// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_header_add_layer_version_permission_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_1: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_1.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub fn deser_header_add_permission_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_2: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_2.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_2 = var_2;
        Ok(var_2.pop())
    }
}

pub fn deser_header_create_alias_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_3: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_3.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_3 = var_3;
        Ok(var_3.pop())
    }
}

pub fn deser_header_create_event_source_mapping_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_4: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_4.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_4 = var_4;
        Ok(var_4.pop())
    }
}

pub fn deser_header_create_function_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_5: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_5.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_5 = var_5;
        Ok(var_5.pop())
    }
}

pub fn deser_header_delete_alias_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_6: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_6.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_6 = var_6;
        Ok(var_6.pop())
    }
}

pub fn deser_header_delete_event_source_mapping_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_7: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_7.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_7 = var_7;
        Ok(var_7.pop())
    }
}

pub fn deser_header_delete_function_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_8: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_8.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_8 = var_8;
        Ok(var_8.pop())
    }
}

pub fn deser_header_delete_function_code_signing_config_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_9: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_9.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_9 = var_9;
        Ok(var_9.pop())
    }
}

pub fn deser_header_delete_function_concurrency_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_10: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_10.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_10 = var_10;
        Ok(var_10.pop())
    }
}

pub fn deser_header_delete_function_event_invoke_config_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_11: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_11.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_11 = var_11;
        Ok(var_11.pop())
    }
}

pub fn deser_header_delete_layer_version_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_12: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_12.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_12 = var_12;
        Ok(var_12.pop())
    }
}

pub fn deser_header_delete_provisioned_concurrency_config_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_13: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_13.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_13 = var_13;
        Ok(var_13.pop())
    }
}

pub fn deser_header_get_account_settings_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_14: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_14.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_14 = var_14;
        Ok(var_14.pop())
    }
}

pub fn deser_header_get_alias_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_15: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_15.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_15 = var_15;
        Ok(var_15.pop())
    }
}

pub fn deser_header_get_event_source_mapping_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_16: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_16.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_16 = var_16;
        Ok(var_16.pop())
    }
}

pub fn deser_header_get_function_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_17: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_17.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_17 = var_17;
        Ok(var_17.pop())
    }
}

pub fn deser_header_get_function_code_signing_config_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_18: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_18.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_18 = var_18;
        Ok(var_18.pop())
    }
}

pub fn deser_header_get_function_concurrency_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_19: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_19.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_19 = var_19;
        Ok(var_19.pop())
    }
}

pub fn deser_header_get_function_configuration_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_20: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_20.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_20 = var_20;
        Ok(var_20.pop())
    }
}

pub fn deser_header_get_function_event_invoke_config_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_21: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_21.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_21 = var_21;
        Ok(var_21.pop())
    }
}

pub fn deser_header_get_layer_version_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_22: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_22.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_22 = var_22;
        Ok(var_22.pop())
    }
}

pub fn deser_header_get_layer_version_by_arn_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_23: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_23.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_23 = var_23;
        Ok(var_23.pop())
    }
}

pub fn deser_header_get_layer_version_policy_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_24: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_24.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_24 = var_24;
        Ok(var_24.pop())
    }
}

pub fn deser_header_get_policy_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_25: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_25.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_25 = var_25;
        Ok(var_25.pop())
    }
}

pub fn deser_header_get_provisioned_concurrency_config_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_26: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_26.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_26 = var_26;
        Ok(var_26.pop())
    }
}

pub fn deser_header_invoke_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_27: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_27.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_27 = var_27;
        Ok(var_27.pop())
    }
}

pub fn deser_header_invoke_function_error(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("X-Amz-Function-Error").iter();
    let var_28: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_28.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_28 = var_28;
        Ok(var_28.pop())
    }
}

pub fn deser_header_invoke_log_result(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("X-Amz-Log-Result").iter();
    let var_29: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_29.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_29 = var_29;
        Ok(var_29.pop())
    }
}

pub fn deser_payload_invoke_payload(
    body: &[u8],
) -> Result<std::option::Option<smithy_types::Blob>, crate::error::InvokeError> {
    (!body.is_empty())
        .then(|| Ok(smithy_types::Blob::new(body)))
        .transpose()
}

pub fn deser_header_invoke_executed_version(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("X-Amz-Executed-Version").iter();
    let var_30: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_30.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_30 = var_30;
        Ok(var_30.pop())
    }
}

pub fn deser_header_list_aliases_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_31: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_31.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_31 = var_31;
        Ok(var_31.pop())
    }
}

pub fn deser_header_list_event_source_mappings_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_32: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_32.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_32 = var_32;
        Ok(var_32.pop())
    }
}

pub fn deser_header_list_function_event_invoke_configs_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_33: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_33.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_33 = var_33;
        Ok(var_33.pop())
    }
}

pub fn deser_header_list_functions_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_34: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_34.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_34 = var_34;
        Ok(var_34.pop())
    }
}

pub fn deser_header_list_layers_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_35: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_35.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_35 = var_35;
        Ok(var_35.pop())
    }
}

pub fn deser_header_list_layer_versions_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_36: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_36.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_36 = var_36;
        Ok(var_36.pop())
    }
}

pub fn deser_header_list_provisioned_concurrency_configs_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_37: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_37.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_37 = var_37;
        Ok(var_37.pop())
    }
}

pub fn deser_header_list_tags_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_38: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_38.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_38 = var_38;
        Ok(var_38.pop())
    }
}

pub fn deser_header_list_versions_by_function_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_39: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_39.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_39 = var_39;
        Ok(var_39.pop())
    }
}

pub fn deser_header_publish_layer_version_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_40: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_40.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_40 = var_40;
        Ok(var_40.pop())
    }
}

pub fn deser_header_publish_version_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_41: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_41.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_41 = var_41;
        Ok(var_41.pop())
    }
}

pub fn deser_header_put_function_code_signing_config_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_42: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_42.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_42 = var_42;
        Ok(var_42.pop())
    }
}

pub fn deser_header_put_function_concurrency_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_43: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_43.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_43 = var_43;
        Ok(var_43.pop())
    }
}

pub fn deser_header_put_function_event_invoke_config_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_44: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_44.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_44 = var_44;
        Ok(var_44.pop())
    }
}

pub fn deser_header_put_provisioned_concurrency_config_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_45: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_45.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_45 = var_45;
        Ok(var_45.pop())
    }
}

pub fn deser_header_remove_layer_version_permission_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_46: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_46.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_46 = var_46;
        Ok(var_46.pop())
    }
}

pub fn deser_header_remove_permission_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_47: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_47.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_47 = var_47;
        Ok(var_47.pop())
    }
}

pub fn deser_header_tag_resource_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_48: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_48.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_48 = var_48;
        Ok(var_48.pop())
    }
}

pub fn deser_header_untag_resource_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_49: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_49.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_49 = var_49;
        Ok(var_49.pop())
    }
}

pub fn deser_header_update_alias_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_50: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_50.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_50 = var_50;
        Ok(var_50.pop())
    }
}

pub fn deser_header_update_event_source_mapping_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_51: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_51.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_51 = var_51;
        Ok(var_51.pop())
    }
}

pub fn deser_header_update_function_code_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_52: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_52.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_52 = var_52;
        Ok(var_52.pop())
    }
}

pub fn deser_header_update_function_configuration_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_53: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_53.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_53 = var_53;
        Ok(var_53.pop())
    }
}

pub fn deser_header_update_function_event_invoke_config_retry_after_seconds(
    header_map: &http::HeaderMap,
) -> Result<std::option::Option<std::string::String>, smithy_http::header::ParseError> {
    let headers = header_map.get_all("Retry-After").iter();
    let var_54: Vec<std::string::String> = smithy_http::header::read_many(headers)?;
    if var_54.len() > 1 {
        Err(smithy_http::header::ParseError)
    } else {
        let mut var_54 = var_54;
        Ok(var_54.pop())
    }
}
