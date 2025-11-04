use bevy::{
    ecs::{system::In, world::World},
    reflect::PartialReflect,
    remote::{
        BrpError, BrpResult,
        error_codes::{INTERNAL_ERROR, INVALID_REQUEST, PARSE_ERROR},
    },
};
use serde::Deserialize;

use crate::simulation::{SimulationConfig, SimulationField};

#[derive(Deserialize)]
struct UpdateSimulationFieldValueRequest {
    pub field_name: String,
    pub value: f32,
}

pub fn simulation_update_field(
    In(params): In<Option<serde_json::Value>>,
    world: &mut World,
) -> BrpResult {
    let Some(value) = params else {
        return Err(BrpError {
            code: INVALID_REQUEST,
            data: None,
            message: "simulation.update_field: Request was empty".to_string(),
        });
    };

    let Ok(request) = serde_json::from_value::<UpdateSimulationFieldValueRequest>(value) else {
        return Err(BrpError {
            code: PARSE_ERROR,
            data: None,
            message: "simulation.update_field: Unable to parse request".to_string(),
        });
    };

    if let Some(mut cfg) = world.get_resource_mut::<SimulationConfig>()
        && let Ok(reflect_struct) = cfg.reflect_mut().as_struct()
        && let Some(field) = reflect_struct.field_mut(&request.field_name)
        && let Some(value) = field.try_downcast_mut::<SimulationField>()
    {
        let old_value = **value;

        **value = request.value;

        return Ok(serde_json::Value::String(format!(
            "Updated SimulationConfig::{}: {} -> {}",
            request.field_name, old_value, request.value
        )));
    }

    return Err(BrpError {
        code: INTERNAL_ERROR,
        message: "simulation.update_field: Unknown field".to_string(),
        data: None,
    });
}
