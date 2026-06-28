//! Recipe-style dry-run reporting for generated migration artifacts.
//!
//! This is intentionally smaller than OpenRewrite's LST runtime: it gives host
//! applications deterministic recipe metadata, before/after text, diagnostics,
//! and a reviewable patch for generated SQL/runbooks without applying anything.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecipePhase {
    Scan,
    Generate,
    Edit,
    Verify,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationRecipe {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub phases: Vec<RecipePhase>,
    pub causes_another_cycle: bool,
}

impl MigrationRecipe {
    pub fn new(
        name: impl Into<String>,
        display_name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            display_name: display_name.into(),
            description: description.into(),
            phases: vec![RecipePhase::Scan, RecipePhase::Edit, RecipePhase::Verify],
            causes_another_cycle: false,
        }
    }

    pub fn with_phases(mut self, phases: Vec<RecipePhase>) -> Self {
        self.phases = phases;
        self
    }

    pub fn causes_another_cycle(mut self) -> Self {
        self.causes_another_cycle = true;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipePreview {
    pub recipe_name: String,
    pub artifact_path: String,
    pub changed: bool,
    pub patch: String,
    pub diagnostics: Vec<String>,
}

pub fn dry_run_text_recipe(
    recipe: &MigrationRecipe,
    artifact_path: impl Into<String>,
    before: &str,
    after: &str,
) -> RecipePreview {
    let artifact_path = artifact_path.into();
    let changed = before != after;
    let patch = if changed {
        unified_line_diff(&artifact_path, before, after)
    } else {
        String::new()
    };
    let mut diagnostics = Vec::new();
    if recipe.causes_another_cycle {
        diagnostics.push("recipe requests another cycle when it changes output".to_string());
    }
    RecipePreview {
        recipe_name: recipe.name.clone(),
        artifact_path,
        changed,
        patch,
        diagnostics,
    }
}

pub fn recipe_run_summary(previews: &[RecipePreview]) -> Vec<String> {
    previews
        .iter()
        .filter(|preview| preview.changed)
        .map(|preview| format!("{} changed {}", preview.recipe_name, preview.artifact_path))
        .collect()
}

fn unified_line_diff(path: &str, before: &str, after: &str) -> String {
    let before_lines = before.lines().collect::<Vec<_>>();
    let after_lines = after.lines().collect::<Vec<_>>();
    let mut lines = vec![format!("--- a/{path}"), format!("+++ b/{path}")];
    let max = before_lines.len().max(after_lines.len());
    for index in 0..max {
        match (before_lines.get(index), after_lines.get(index)) {
            (Some(left), Some(right)) if left == right => lines.push(format!(" {left}")),
            (Some(left), Some(right)) => {
                lines.push(format!("-{left}"));
                lines.push(format!("+{right}"));
            }
            (Some(left), None) => lines.push(format!("-{left}")),
            (None, Some(right)) => lines.push(format!("+{right}")),
            (None, None) => {}
        }
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unchanged_recipe_has_no_patch() {
        let recipe = MigrationRecipe::new(
            "irodori.sql.format",
            "Format SQL",
            "Normalize generated SQL.",
        );
        let preview = dry_run_text_recipe(&recipe, "plan.sql", "select 1;\n", "select 1;\n");

        assert!(!preview.changed);
        assert!(preview.patch.is_empty());
    }

    #[test]
    fn changed_recipe_reports_patch_and_summary() {
        let recipe = MigrationRecipe::new(
            "irodori.sql.rewrite",
            "Rewrite SQL",
            "Rewrite generated SQL.",
        )
        .causes_another_cycle();
        let preview = dry_run_text_recipe(&recipe, "plan.sql", "select 1;", "select 2;");

        assert!(preview.changed);
        assert!(preview.patch.contains("--- a/plan.sql"));
        assert!(preview.patch.contains("-select 1;"));
        assert!(preview.patch.contains("+select 2;"));
        assert_eq!(
            recipe_run_summary(&[preview]),
            vec!["irodori.sql.rewrite changed plan.sql".to_string()]
        );
    }
}
