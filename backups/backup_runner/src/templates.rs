use anyhow::Error;
use rustic_core::repofile::SnapshotSummary;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "report.stpl")]
pub struct ReportTemplate {
    pub summaries: Vec<(String, SnapshotSummary)>,
    pub errors: Vec<(String, Error)>,
}
