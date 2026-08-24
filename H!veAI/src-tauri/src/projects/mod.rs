mod detection;
mod paths;
mod registry;

pub use registry::{
    archive_project, fetch_project, list_projects, register_project, remove_project,
    repair_project_path, update_project_settings, ProjectListQuery, ProjectRecord,
    RegisterProjectRequest, RepairProjectPathRequest, UpdateProjectSettingsRequest,
};
