// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn cancel_key_deletion_deser_operation(
    inp: &[u8],
    mut builder: crate::output::cancel_key_deletion_output::Builder,
) -> Result<crate::output::cancel_key_deletion_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::CancelKeyDeletionOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_key_id(parsed_body.key_id);
    Ok(builder)
}

pub fn create_custom_key_store_deser_operation(
    inp: &[u8],
    mut builder: crate::output::create_custom_key_store_output::Builder,
) -> Result<crate::output::create_custom_key_store_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::CreateCustomKeyStoreOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_custom_key_store_id(parsed_body.custom_key_store_id);
    Ok(builder)
}

pub fn create_grant_deser_operation(
    inp: &[u8],
    mut builder: crate::output::create_grant_output::Builder,
) -> Result<crate::output::create_grant_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::CreateGrantOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_grant_token(parsed_body.grant_token);
    builder = builder.set_grant_id(parsed_body.grant_id);
    Ok(builder)
}

pub fn create_key_deser_operation(
    inp: &[u8],
    mut builder: crate::output::create_key_output::Builder,
) -> Result<crate::output::create_key_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::CreateKeyOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_key_metadata(parsed_body.key_metadata);
    Ok(builder)
}

pub fn decrypt_deser_operation(
    inp: &[u8],
    mut builder: crate::output::decrypt_output::Builder,
) -> Result<crate::output::decrypt_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DecryptOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_key_id(parsed_body.key_id);
    builder = builder.set_plaintext(parsed_body.plaintext);
    builder = builder.set_encryption_algorithm(parsed_body.encryption_algorithm);
    Ok(builder)
}

pub fn describe_custom_key_stores_deser_operation(
    inp: &[u8],
    mut builder: crate::output::describe_custom_key_stores_output::Builder,
) -> Result<crate::output::describe_custom_key_stores_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeCustomKeyStoresOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_custom_key_stores(parsed_body.custom_key_stores);
    builder = builder.set_next_marker(parsed_body.next_marker);
    builder = builder.set_truncated(parsed_body.truncated);
    Ok(builder)
}

pub fn describe_key_deser_operation(
    inp: &[u8],
    mut builder: crate::output::describe_key_output::Builder,
) -> Result<crate::output::describe_key_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::DescribeKeyOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_key_metadata(parsed_body.key_metadata);
    Ok(builder)
}

pub fn encrypt_deser_operation(
    inp: &[u8],
    mut builder: crate::output::encrypt_output::Builder,
) -> Result<crate::output::encrypt_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::EncryptOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_ciphertext_blob(parsed_body.ciphertext_blob);
    builder = builder.set_key_id(parsed_body.key_id);
    builder = builder.set_encryption_algorithm(parsed_body.encryption_algorithm);
    Ok(builder)
}

pub fn generate_data_key_deser_operation(
    inp: &[u8],
    mut builder: crate::output::generate_data_key_output::Builder,
) -> Result<crate::output::generate_data_key_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GenerateDataKeyOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_ciphertext_blob(parsed_body.ciphertext_blob);
    builder = builder.set_plaintext(parsed_body.plaintext);
    builder = builder.set_key_id(parsed_body.key_id);
    Ok(builder)
}

pub fn generate_data_key_pair_deser_operation(
    inp: &[u8],
    mut builder: crate::output::generate_data_key_pair_output::Builder,
) -> Result<crate::output::generate_data_key_pair_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GenerateDataKeyPairOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_private_key_ciphertext_blob(parsed_body.private_key_ciphertext_blob);
    builder = builder.set_private_key_plaintext(parsed_body.private_key_plaintext);
    builder = builder.set_public_key(parsed_body.public_key);
    builder = builder.set_key_id(parsed_body.key_id);
    builder = builder.set_key_pair_spec(parsed_body.key_pair_spec);
    Ok(builder)
}

pub fn generate_data_key_pair_without_plaintext_deser_operation(
    inp: &[u8],
    mut builder: crate::output::generate_data_key_pair_without_plaintext_output::Builder,
) -> Result<
    crate::output::generate_data_key_pair_without_plaintext_output::Builder,
    serde_json::Error,
> {
    let parsed_body: crate::serializer::GenerateDataKeyPairWithoutPlaintextOutputBody =
        if inp.is_empty() {
            // To enable JSON parsing to succeed, replace an empty body
            // with an empty JSON body. If a member was required, it will fail slightly later
            // during the operation construction phase when a required field was missing.
            serde_json::from_slice(b"{}")?
        } else {
            serde_json::from_slice(inp)?
        };
    builder = builder.set_private_key_ciphertext_blob(parsed_body.private_key_ciphertext_blob);
    builder = builder.set_public_key(parsed_body.public_key);
    builder = builder.set_key_id(parsed_body.key_id);
    builder = builder.set_key_pair_spec(parsed_body.key_pair_spec);
    Ok(builder)
}

pub fn generate_data_key_without_plaintext_deser_operation(
    inp: &[u8],
    mut builder: crate::output::generate_data_key_without_plaintext_output::Builder,
) -> Result<crate::output::generate_data_key_without_plaintext_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GenerateDataKeyWithoutPlaintextOutputBody =
        if inp.is_empty() {
            // To enable JSON parsing to succeed, replace an empty body
            // with an empty JSON body. If a member was required, it will fail slightly later
            // during the operation construction phase when a required field was missing.
            serde_json::from_slice(b"{}")?
        } else {
            serde_json::from_slice(inp)?
        };
    builder = builder.set_ciphertext_blob(parsed_body.ciphertext_blob);
    builder = builder.set_key_id(parsed_body.key_id);
    Ok(builder)
}

pub fn generate_random_deser_operation(
    inp: &[u8],
    mut builder: crate::output::generate_random_output::Builder,
) -> Result<crate::output::generate_random_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GenerateRandomOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_plaintext(parsed_body.plaintext);
    Ok(builder)
}

pub fn get_key_policy_deser_operation(
    inp: &[u8],
    mut builder: crate::output::get_key_policy_output::Builder,
) -> Result<crate::output::get_key_policy_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GetKeyPolicyOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_policy(parsed_body.policy);
    Ok(builder)
}

pub fn get_key_rotation_status_deser_operation(
    inp: &[u8],
    mut builder: crate::output::get_key_rotation_status_output::Builder,
) -> Result<crate::output::get_key_rotation_status_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GetKeyRotationStatusOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_key_rotation_enabled(parsed_body.key_rotation_enabled);
    Ok(builder)
}

pub fn get_parameters_for_import_deser_operation(
    inp: &[u8],
    mut builder: crate::output::get_parameters_for_import_output::Builder,
) -> Result<crate::output::get_parameters_for_import_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GetParametersForImportOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_key_id(parsed_body.key_id);
    builder = builder.set_import_token(parsed_body.import_token);
    builder = builder.set_public_key(parsed_body.public_key);
    builder = builder.set_parameters_valid_to(parsed_body.parameters_valid_to);
    Ok(builder)
}

pub fn get_public_key_deser_operation(
    inp: &[u8],
    mut builder: crate::output::get_public_key_output::Builder,
) -> Result<crate::output::get_public_key_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::GetPublicKeyOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_key_id(parsed_body.key_id);
    builder = builder.set_public_key(parsed_body.public_key);
    builder = builder.set_customer_master_key_spec(parsed_body.customer_master_key_spec);
    builder = builder.set_key_usage(parsed_body.key_usage);
    builder = builder.set_encryption_algorithms(parsed_body.encryption_algorithms);
    builder = builder.set_signing_algorithms(parsed_body.signing_algorithms);
    Ok(builder)
}

pub fn list_aliases_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_aliases_output::Builder,
) -> Result<crate::output::list_aliases_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListAliasesOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_aliases(parsed_body.aliases);
    builder = builder.set_next_marker(parsed_body.next_marker);
    builder = builder.set_truncated(parsed_body.truncated);
    Ok(builder)
}

pub fn list_grants_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_grants_output::Builder,
) -> Result<crate::output::list_grants_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListGrantsOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_grants(parsed_body.grants);
    builder = builder.set_next_marker(parsed_body.next_marker);
    builder = builder.set_truncated(parsed_body.truncated);
    Ok(builder)
}

pub fn list_key_policies_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_key_policies_output::Builder,
) -> Result<crate::output::list_key_policies_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListKeyPoliciesOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_policy_names(parsed_body.policy_names);
    builder = builder.set_next_marker(parsed_body.next_marker);
    builder = builder.set_truncated(parsed_body.truncated);
    Ok(builder)
}

pub fn list_keys_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_keys_output::Builder,
) -> Result<crate::output::list_keys_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListKeysOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_keys(parsed_body.keys);
    builder = builder.set_next_marker(parsed_body.next_marker);
    builder = builder.set_truncated(parsed_body.truncated);
    Ok(builder)
}

pub fn list_resource_tags_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_resource_tags_output::Builder,
) -> Result<crate::output::list_resource_tags_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListResourceTagsOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_tags(parsed_body.tags);
    builder = builder.set_next_marker(parsed_body.next_marker);
    builder = builder.set_truncated(parsed_body.truncated);
    Ok(builder)
}

pub fn list_retirable_grants_deser_operation(
    inp: &[u8],
    mut builder: crate::output::list_retirable_grants_output::Builder,
) -> Result<crate::output::list_retirable_grants_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ListRetirableGrantsOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_grants(parsed_body.grants);
    builder = builder.set_next_marker(parsed_body.next_marker);
    builder = builder.set_truncated(parsed_body.truncated);
    Ok(builder)
}

pub fn re_encrypt_deser_operation(
    inp: &[u8],
    mut builder: crate::output::re_encrypt_output::Builder,
) -> Result<crate::output::re_encrypt_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ReEncryptOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_ciphertext_blob(parsed_body.ciphertext_blob);
    builder = builder.set_source_key_id(parsed_body.source_key_id);
    builder = builder.set_key_id(parsed_body.key_id);
    builder = builder.set_source_encryption_algorithm(parsed_body.source_encryption_algorithm);
    builder =
        builder.set_destination_encryption_algorithm(parsed_body.destination_encryption_algorithm);
    Ok(builder)
}

pub fn schedule_key_deletion_deser_operation(
    inp: &[u8],
    mut builder: crate::output::schedule_key_deletion_output::Builder,
) -> Result<crate::output::schedule_key_deletion_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::ScheduleKeyDeletionOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_key_id(parsed_body.key_id);
    builder = builder.set_deletion_date(parsed_body.deletion_date);
    Ok(builder)
}

pub fn sign_deser_operation(
    inp: &[u8],
    mut builder: crate::output::sign_output::Builder,
) -> Result<crate::output::sign_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::SignOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_key_id(parsed_body.key_id);
    builder = builder.set_signature(parsed_body.signature);
    builder = builder.set_signing_algorithm(parsed_body.signing_algorithm);
    Ok(builder)
}

pub fn verify_deser_operation(
    inp: &[u8],
    mut builder: crate::output::verify_output::Builder,
) -> Result<crate::output::verify_output::Builder, serde_json::Error> {
    let parsed_body: crate::serializer::VerifyOutputBody = if inp.is_empty() {
        // To enable JSON parsing to succeed, replace an empty body
        // with an empty JSON body. If a member was required, it will fail slightly later
        // during the operation construction phase when a required field was missing.
        serde_json::from_slice(b"{}")?
    } else {
        serde_json::from_slice(inp)?
    };
    builder = builder.set_key_id(parsed_body.key_id);
    builder = builder.set_signature_valid(parsed_body.signature_valid);
    builder = builder.set_signing_algorithm(parsed_body.signing_algorithm);
    Ok(builder)
}
