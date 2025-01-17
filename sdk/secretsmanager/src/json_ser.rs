// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_cancel_rotate_secret_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelRotateSecretInput,
) {
    if let Some(var_1) = &input.secret_id {
        object.key("SecretId").string(var_1);
    }
}

pub fn serialize_structure_create_secret_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSecretInput,
) {
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2);
    }
    if let Some(var_3) = &input.tags {
        let mut array_4 = object.key("Tags").start_array();
        for item_5 in var_3 {
            let mut object_6 = array_4.value().start_object();
            crate::json_ser::serialize_structure_tag(&mut object_6, item_5);
            object_6.finish();
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_7);
    }
    if let Some(var_8) = &input.description {
        object.key("Description").string(var_8);
    }
    if let Some(var_9) = &input.secret_string {
        object.key("SecretString").string(var_9);
    }
    if let Some(var_10) = &input.secret_binary {
        object
            .key("SecretBinary")
            .string_unchecked(&smithy_http::base64::encode(var_10));
    }
    if let Some(var_11) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_11);
    }
}

pub fn serialize_structure_delete_resource_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteResourcePolicyInput,
) {
    if let Some(var_12) = &input.secret_id {
        object.key("SecretId").string(var_12);
    }
}

pub fn serialize_structure_delete_secret_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSecretInput,
) {
    if let Some(var_13) = &input.force_delete_without_recovery {
        object.key("ForceDeleteWithoutRecovery").boolean(*var_13);
    }
    if let Some(var_14) = &input.recovery_window_in_days {
        object.key("RecoveryWindowInDays").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    if let Some(var_15) = &input.secret_id {
        object.key("SecretId").string(var_15);
    }
}

pub fn serialize_structure_describe_secret_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSecretInput,
) {
    if let Some(var_16) = &input.secret_id {
        object.key("SecretId").string(var_16);
    }
}

pub fn serialize_structure_get_random_password_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRandomPasswordInput,
) {
    if let Some(var_17) = &input.exclude_numbers {
        object.key("ExcludeNumbers").boolean(*var_17);
    }
    if let Some(var_18) = &input.exclude_characters {
        object.key("ExcludeCharacters").string(var_18);
    }
    if let Some(var_19) = &input.password_length {
        object.key("PasswordLength").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_19).into()),
        );
    }
    if let Some(var_20) = &input.require_each_included_type {
        object.key("RequireEachIncludedType").boolean(*var_20);
    }
    if let Some(var_21) = &input.include_space {
        object.key("IncludeSpace").boolean(*var_21);
    }
    if let Some(var_22) = &input.exclude_punctuation {
        object.key("ExcludePunctuation").boolean(*var_22);
    }
    if let Some(var_23) = &input.exclude_lowercase {
        object.key("ExcludeLowercase").boolean(*var_23);
    }
    if let Some(var_24) = &input.exclude_uppercase {
        object.key("ExcludeUppercase").boolean(*var_24);
    }
}

pub fn serialize_structure_get_resource_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetResourcePolicyInput,
) {
    if let Some(var_25) = &input.secret_id {
        object.key("SecretId").string(var_25);
    }
}

pub fn serialize_structure_get_secret_value_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetSecretValueInput,
) {
    if let Some(var_26) = &input.secret_id {
        object.key("SecretId").string(var_26);
    }
    if let Some(var_27) = &input.version_stage {
        object.key("VersionStage").string(var_27);
    }
    if let Some(var_28) = &input.version_id {
        object.key("VersionId").string(var_28);
    }
}

pub fn serialize_structure_list_secrets_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSecretsInput,
) {
    if let Some(var_29) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_29).into()),
        );
    }
    if let Some(var_30) = &input.sort_order {
        object.key("SortOrder").string(var_30.as_str());
    }
    if let Some(var_31) = &input.next_token {
        object.key("NextToken").string(var_31);
    }
    if let Some(var_32) = &input.filters {
        let mut array_33 = object.key("Filters").start_array();
        for item_34 in var_32 {
            let mut object_35 = array_33.value().start_object();
            crate::json_ser::serialize_structure_filter(&mut object_35, item_34);
            object_35.finish();
        }
        array_33.finish();
    }
}

pub fn serialize_structure_list_secret_version_ids_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSecretVersionIdsInput,
) {
    if let Some(var_36) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_36).into()),
        );
    }
    if let Some(var_37) = &input.secret_id {
        object.key("SecretId").string(var_37);
    }
    if let Some(var_38) = &input.next_token {
        object.key("NextToken").string(var_38);
    }
    if let Some(var_39) = &input.include_deprecated {
        object.key("IncludeDeprecated").boolean(*var_39);
    }
}

pub fn serialize_structure_put_resource_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutResourcePolicyInput,
) {
    if let Some(var_40) = &input.resource_policy {
        object.key("ResourcePolicy").string(var_40);
    }
    if let Some(var_41) = &input.block_public_policy {
        object.key("BlockPublicPolicy").boolean(*var_41);
    }
    if let Some(var_42) = &input.secret_id {
        object.key("SecretId").string(var_42);
    }
}

pub fn serialize_structure_put_secret_value_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutSecretValueInput,
) {
    if let Some(var_43) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_43);
    }
    if let Some(var_44) = &input.secret_id {
        object.key("SecretId").string(var_44);
    }
    if let Some(var_45) = &input.secret_string {
        object.key("SecretString").string(var_45);
    }
    if let Some(var_46) = &input.secret_binary {
        object
            .key("SecretBinary")
            .string_unchecked(&smithy_http::base64::encode(var_46));
    }
    if let Some(var_47) = &input.version_stages {
        let mut array_48 = object.key("VersionStages").start_array();
        for item_49 in var_47 {
            array_48.value().string(item_49);
        }
        array_48.finish();
    }
}

pub fn serialize_structure_restore_secret_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RestoreSecretInput,
) {
    if let Some(var_50) = &input.secret_id {
        object.key("SecretId").string(var_50);
    }
}

pub fn serialize_structure_rotate_secret_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RotateSecretInput,
) {
    if let Some(var_51) = &input.secret_id {
        object.key("SecretId").string(var_51);
    }
    if let Some(var_52) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_52);
    }
    if let Some(var_53) = &input.rotation_rules {
        let mut object_54 = object.key("RotationRules").start_object();
        crate::json_ser::serialize_structure_rotation_rules_type(&mut object_54, var_53);
        object_54.finish();
    }
    if let Some(var_55) = &input.rotation_lambda_arn {
        object.key("RotationLambdaARN").string(var_55);
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_56) = &input.secret_id {
        object.key("SecretId").string(var_56);
    }
    if let Some(var_57) = &input.tags {
        let mut array_58 = object.key("Tags").start_array();
        for item_59 in var_57 {
            let mut object_60 = array_58.value().start_object();
            crate::json_ser::serialize_structure_tag(&mut object_60, item_59);
            object_60.finish();
        }
        array_58.finish();
    }
}

pub fn serialize_structure_untag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) {
    if let Some(var_61) = &input.tag_keys {
        let mut array_62 = object.key("TagKeys").start_array();
        for item_63 in var_61 {
            array_62.value().string(item_63);
        }
        array_62.finish();
    }
    if let Some(var_64) = &input.secret_id {
        object.key("SecretId").string(var_64);
    }
}

pub fn serialize_structure_update_secret_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSecretInput,
) {
    if let Some(var_65) = &input.secret_string {
        object.key("SecretString").string(var_65);
    }
    if let Some(var_66) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_66);
    }
    if let Some(var_67) = &input.secret_id {
        object.key("SecretId").string(var_67);
    }
    if let Some(var_68) = &input.description {
        object.key("Description").string(var_68);
    }
    if let Some(var_69) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_69);
    }
    if let Some(var_70) = &input.secret_binary {
        object
            .key("SecretBinary")
            .string_unchecked(&smithy_http::base64::encode(var_70));
    }
}

pub fn serialize_structure_update_secret_version_stage_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSecretVersionStageInput,
) {
    if let Some(var_71) = &input.move_to_version_id {
        object.key("MoveToVersionId").string(var_71);
    }
    if let Some(var_72) = &input.secret_id {
        object.key("SecretId").string(var_72);
    }
    if let Some(var_73) = &input.remove_from_version_id {
        object.key("RemoveFromVersionId").string(var_73);
    }
    if let Some(var_74) = &input.version_stage {
        object.key("VersionStage").string(var_74);
    }
}

pub fn serialize_structure_validate_resource_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ValidateResourcePolicyInput,
) {
    if let Some(var_75) = &input.resource_policy {
        object.key("ResourcePolicy").string(var_75);
    }
    if let Some(var_76) = &input.secret_id {
        object.key("SecretId").string(var_76);
    }
}

pub fn serialize_structure_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_77) = &input.value {
        object.key("Value").string(var_77);
    }
    if let Some(var_78) = &input.key {
        object.key("Key").string(var_78);
    }
}

pub fn serialize_structure_filter(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) {
    if let Some(var_79) = &input.key {
        object.key("Key").string(var_79.as_str());
    }
    if let Some(var_80) = &input.values {
        let mut array_81 = object.key("Values").start_array();
        for item_82 in var_80 {
            array_81.value().string(item_82);
        }
        array_81.finish();
    }
}

pub fn serialize_structure_rotation_rules_type(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RotationRulesType,
) {
    if let Some(var_83) = &input.automatically_after_days {
        object.key("AutomaticallyAfterDays").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_83).into()),
        );
    }
}
