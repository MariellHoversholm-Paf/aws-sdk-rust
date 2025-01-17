// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_add_permission_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::AddPermissionOutput, crate::error::AddPermissionError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::AddPermissionError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::AddPermissionError::unhandled(generic)),
    };
    Err(match error_code {
        "OverLimit" => crate::error::AddPermissionError {
            meta: generic,
            kind: crate::error::AddPermissionErrorKind::OverLimit({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::over_limit::Builder::default();
                output = crate::xml_deser::over_limit(response.body().as_ref(), output)
                    .map_err(crate::error::AddPermissionError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::AddPermissionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_add_permission_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::AddPermissionOutput, crate::error::AddPermissionError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::add_permission_output::Builder::default();
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_change_message_visibility_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::ChangeMessageVisibilityOutput, crate::error::ChangeMessageVisibilityError>
{
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::ChangeMessageVisibilityError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::ChangeMessageVisibilityError::unhandled(
                generic,
            ))
        }
    };
    Err(match error_code {
        "MessageNotInflight" => crate::error::ChangeMessageVisibilityError {
            meta: generic,
            kind: crate::error::ChangeMessageVisibilityErrorKind::MessageNotInflight({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::message_not_inflight::Builder::default();
                output = crate::xml_deser::message_not_inflight(response.body().as_ref(), output)
                    .map_err(crate::error::ChangeMessageVisibilityError::unhandled)?;
                output.build()
            }),
        },
        "ReceiptHandleIsInvalid" => crate::error::ChangeMessageVisibilityError {
            meta: generic,
            kind: crate::error::ChangeMessageVisibilityErrorKind::ReceiptHandleIsInvalid({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::receipt_handle_is_invalid::Builder::default();
                output =
                    crate::xml_deser::receipt_handle_is_invalid(response.body().as_ref(), output)
                        .map_err(crate::error::ChangeMessageVisibilityError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::ChangeMessageVisibilityError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_change_message_visibility_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::ChangeMessageVisibilityOutput, crate::error::ChangeMessageVisibilityError>
{
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::change_message_visibility_output::Builder::default();
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_change_message_visibility_batch_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    crate::output::ChangeMessageVisibilityBatchOutput,
    crate::error::ChangeMessageVisibilityBatchError,
> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::ChangeMessageVisibilityBatchError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::ChangeMessageVisibilityBatchError::unhandled(
                generic,
            ))
        }
    };
    Err(match error_code {
        "BatchEntryIdsNotDistinct" => crate::error::ChangeMessageVisibilityBatchError {
            meta: generic,
            kind: crate::error::ChangeMessageVisibilityBatchErrorKind::BatchEntryIdsNotDistinct({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::batch_entry_ids_not_distinct::Builder::default();
                output = crate::xml_deser::batch_entry_ids_not_distinct(
                    response.body().as_ref(),
                    output,
                )
                .map_err(crate::error::ChangeMessageVisibilityBatchError::unhandled)?;
                output.build()
            }),
        },
        "EmptyBatchRequest" => crate::error::ChangeMessageVisibilityBatchError {
            meta: generic,
            kind: crate::error::ChangeMessageVisibilityBatchErrorKind::EmptyBatchRequest({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::empty_batch_request::Builder::default();
                output = crate::xml_deser::empty_batch_request(response.body().as_ref(), output)
                    .map_err(crate::error::ChangeMessageVisibilityBatchError::unhandled)?;
                output.build()
            }),
        },
        "InvalidBatchEntryId" => crate::error::ChangeMessageVisibilityBatchError {
            meta: generic,
            kind: crate::error::ChangeMessageVisibilityBatchErrorKind::InvalidBatchEntryId({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::invalid_batch_entry_id::Builder::default();
                output = crate::xml_deser::invalid_batch_entry_id(response.body().as_ref(), output)
                    .map_err(crate::error::ChangeMessageVisibilityBatchError::unhandled)?;
                output.build()
            }),
        },
        "TooManyEntriesInBatchRequest" => crate::error::ChangeMessageVisibilityBatchError {
            meta: generic,
            kind: crate::error::ChangeMessageVisibilityBatchErrorKind::TooManyEntriesInBatchRequest(
                {
                    let _ = response;
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::too_many_entries_in_batch_request::Builder::default();
                    output = crate::xml_deser::too_many_entries_in_batch_request(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::ChangeMessageVisibilityBatchError::unhandled)?;
                    output.build()
                },
            ),
        },
        _ => crate::error::ChangeMessageVisibilityBatchError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_change_message_visibility_batch_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    crate::output::ChangeMessageVisibilityBatchOutput,
    crate::error::ChangeMessageVisibilityBatchError,
> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::change_message_visibility_batch_output::Builder::default();
        output = crate::xml_deser::deser_operation_change_message_visibility_batch(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::ChangeMessageVisibilityBatchError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_create_queue_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::CreateQueueOutput, crate::error::CreateQueueError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::CreateQueueError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::CreateQueueError::unhandled(generic)),
    };
    Err(match error_code {
        "QueueDeletedRecently" => crate::error::CreateQueueError {
            meta: generic,
            kind: crate::error::CreateQueueErrorKind::QueueDeletedRecently({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::queue_deleted_recently::Builder::default();
                output = crate::xml_deser::queue_deleted_recently(response.body().as_ref(), output)
                    .map_err(crate::error::CreateQueueError::unhandled)?;
                output.build()
            }),
        },
        "QueueNameExists" => crate::error::CreateQueueError {
            meta: generic,
            kind: crate::error::CreateQueueErrorKind::QueueNameExists({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::queue_name_exists::Builder::default();
                output = crate::xml_deser::queue_name_exists(response.body().as_ref(), output)
                    .map_err(crate::error::CreateQueueError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::CreateQueueError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_create_queue_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::CreateQueueOutput, crate::error::CreateQueueError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::create_queue_output::Builder::default();
        output = crate::xml_deser::deser_operation_create_queue(response.body().as_ref(), output)
            .map_err(crate::error::CreateQueueError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_message_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::DeleteMessageOutput, crate::error::DeleteMessageError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::DeleteMessageError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DeleteMessageError::unhandled(generic)),
    };
    Err(match error_code {
        "InvalidIdFormat" => crate::error::DeleteMessageError {
            meta: generic,
            kind: crate::error::DeleteMessageErrorKind::InvalidIdFormat({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::invalid_id_format::Builder::default();
                output = crate::xml_deser::invalid_id_format(response.body().as_ref(), output)
                    .map_err(crate::error::DeleteMessageError::unhandled)?;
                output.build()
            }),
        },
        "ReceiptHandleIsInvalid" => crate::error::DeleteMessageError {
            meta: generic,
            kind: crate::error::DeleteMessageErrorKind::ReceiptHandleIsInvalid({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::receipt_handle_is_invalid::Builder::default();
                output =
                    crate::xml_deser::receipt_handle_is_invalid(response.body().as_ref(), output)
                        .map_err(crate::error::DeleteMessageError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::DeleteMessageError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_message_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::DeleteMessageOutput, crate::error::DeleteMessageError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::delete_message_output::Builder::default();
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_message_batch_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::DeleteMessageBatchOutput, crate::error::DeleteMessageBatchError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::DeleteMessageBatchError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DeleteMessageBatchError::unhandled(generic)),
    };
    Err(match error_code {
        "BatchEntryIdsNotDistinct" => crate::error::DeleteMessageBatchError {
            meta: generic,
            kind: crate::error::DeleteMessageBatchErrorKind::BatchEntryIdsNotDistinct({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::batch_entry_ids_not_distinct::Builder::default();
                output = crate::xml_deser::batch_entry_ids_not_distinct(
                    response.body().as_ref(),
                    output,
                )
                .map_err(crate::error::DeleteMessageBatchError::unhandled)?;
                output.build()
            }),
        },
        "EmptyBatchRequest" => crate::error::DeleteMessageBatchError {
            meta: generic,
            kind: crate::error::DeleteMessageBatchErrorKind::EmptyBatchRequest({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::empty_batch_request::Builder::default();
                output = crate::xml_deser::empty_batch_request(response.body().as_ref(), output)
                    .map_err(crate::error::DeleteMessageBatchError::unhandled)?;
                output.build()
            }),
        },
        "InvalidBatchEntryId" => crate::error::DeleteMessageBatchError {
            meta: generic,
            kind: crate::error::DeleteMessageBatchErrorKind::InvalidBatchEntryId({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::invalid_batch_entry_id::Builder::default();
                output = crate::xml_deser::invalid_batch_entry_id(response.body().as_ref(), output)
                    .map_err(crate::error::DeleteMessageBatchError::unhandled)?;
                output.build()
            }),
        },
        "TooManyEntriesInBatchRequest" => crate::error::DeleteMessageBatchError {
            meta: generic,
            kind: crate::error::DeleteMessageBatchErrorKind::TooManyEntriesInBatchRequest({
                let _ = response;
                #[allow(unused_mut)]
                let mut output =
                    crate::error::too_many_entries_in_batch_request::Builder::default();
                output = crate::xml_deser::too_many_entries_in_batch_request(
                    response.body().as_ref(),
                    output,
                )
                .map_err(crate::error::DeleteMessageBatchError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::DeleteMessageBatchError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_message_batch_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::DeleteMessageBatchOutput, crate::error::DeleteMessageBatchError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::delete_message_batch_output::Builder::default();
        output = crate::xml_deser::deser_operation_delete_message_batch(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DeleteMessageBatchError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_queue_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::DeleteQueueOutput, crate::error::DeleteQueueError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::DeleteQueueError::unhandled)?;
    Err(crate::error::DeleteQueueError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_queue_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::DeleteQueueOutput, crate::error::DeleteQueueError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::delete_queue_output::Builder::default();
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_queue_attributes_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::GetQueueAttributesOutput, crate::error::GetQueueAttributesError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::GetQueueAttributesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetQueueAttributesError::unhandled(generic)),
    };
    Err(match error_code {
        "InvalidAttributeName" => crate::error::GetQueueAttributesError {
            meta: generic,
            kind: crate::error::GetQueueAttributesErrorKind::InvalidAttributeName({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::invalid_attribute_name::Builder::default();
                output = crate::xml_deser::invalid_attribute_name(response.body().as_ref(), output)
                    .map_err(crate::error::GetQueueAttributesError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::GetQueueAttributesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_queue_attributes_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::GetQueueAttributesOutput, crate::error::GetQueueAttributesError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::get_queue_attributes_output::Builder::default();
        output = crate::xml_deser::deser_operation_get_queue_attributes(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetQueueAttributesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_queue_url_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::GetQueueUrlOutput, crate::error::GetQueueUrlError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::GetQueueUrlError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetQueueUrlError::unhandled(generic)),
    };
    Err(match error_code {
        "QueueDoesNotExist" => crate::error::GetQueueUrlError {
            meta: generic,
            kind: crate::error::GetQueueUrlErrorKind::QueueDoesNotExist({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::queue_does_not_exist::Builder::default();
                output = crate::xml_deser::queue_does_not_exist(response.body().as_ref(), output)
                    .map_err(crate::error::GetQueueUrlError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::GetQueueUrlError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_queue_url_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::GetQueueUrlOutput, crate::error::GetQueueUrlError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::get_queue_url_output::Builder::default();
        output = crate::xml_deser::deser_operation_get_queue_url(response.body().as_ref(), output)
            .map_err(crate::error::GetQueueUrlError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_dead_letter_source_queues_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    crate::output::ListDeadLetterSourceQueuesOutput,
    crate::error::ListDeadLetterSourceQueuesError,
> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::ListDeadLetterSourceQueuesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::ListDeadLetterSourceQueuesError::unhandled(
                generic,
            ))
        }
    };
    Err(match error_code {
        "QueueDoesNotExist" => crate::error::ListDeadLetterSourceQueuesError {
            meta: generic,
            kind: crate::error::ListDeadLetterSourceQueuesErrorKind::QueueDoesNotExist({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::queue_does_not_exist::Builder::default();
                output = crate::xml_deser::queue_does_not_exist(response.body().as_ref(), output)
                    .map_err(crate::error::ListDeadLetterSourceQueuesError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::ListDeadLetterSourceQueuesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_dead_letter_source_queues_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    crate::output::ListDeadLetterSourceQueuesOutput,
    crate::error::ListDeadLetterSourceQueuesError,
> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::list_dead_letter_source_queues_output::Builder::default();
        output = crate::xml_deser::deser_operation_list_dead_letter_source_queues(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::ListDeadLetterSourceQueuesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_queues_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::ListQueuesOutput, crate::error::ListQueuesError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::ListQueuesError::unhandled)?;
    Err(crate::error::ListQueuesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_queues_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::ListQueuesOutput, crate::error::ListQueuesError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::list_queues_output::Builder::default();
        output = crate::xml_deser::deser_operation_list_queues(response.body().as_ref(), output)
            .map_err(crate::error::ListQueuesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_queue_tags_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::ListQueueTagsOutput, crate::error::ListQueueTagsError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::ListQueueTagsError::unhandled)?;
    Err(crate::error::ListQueueTagsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_queue_tags_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::ListQueueTagsOutput, crate::error::ListQueueTagsError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::list_queue_tags_output::Builder::default();
        output =
            crate::xml_deser::deser_operation_list_queue_tags(response.body().as_ref(), output)
                .map_err(crate::error::ListQueueTagsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_purge_queue_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::PurgeQueueOutput, crate::error::PurgeQueueError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::PurgeQueueError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::PurgeQueueError::unhandled(generic)),
    };
    Err(match error_code {
        "PurgeQueueInProgress" => crate::error::PurgeQueueError {
            meta: generic,
            kind: crate::error::PurgeQueueErrorKind::PurgeQueueInProgress({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::purge_queue_in_progress::Builder::default();
                output =
                    crate::xml_deser::purge_queue_in_progress(response.body().as_ref(), output)
                        .map_err(crate::error::PurgeQueueError::unhandled)?;
                output.build()
            }),
        },
        "QueueDoesNotExist" => crate::error::PurgeQueueError {
            meta: generic,
            kind: crate::error::PurgeQueueErrorKind::QueueDoesNotExist({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::queue_does_not_exist::Builder::default();
                output = crate::xml_deser::queue_does_not_exist(response.body().as_ref(), output)
                    .map_err(crate::error::PurgeQueueError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::PurgeQueueError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_purge_queue_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::PurgeQueueOutput, crate::error::PurgeQueueError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::purge_queue_output::Builder::default();
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_receive_message_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::ReceiveMessageOutput, crate::error::ReceiveMessageError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::ReceiveMessageError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ReceiveMessageError::unhandled(generic)),
    };
    Err(match error_code {
        "OverLimit" => crate::error::ReceiveMessageError {
            meta: generic,
            kind: crate::error::ReceiveMessageErrorKind::OverLimit({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::over_limit::Builder::default();
                output = crate::xml_deser::over_limit(response.body().as_ref(), output)
                    .map_err(crate::error::ReceiveMessageError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::ReceiveMessageError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_receive_message_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::ReceiveMessageOutput, crate::error::ReceiveMessageError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::receive_message_output::Builder::default();
        output =
            crate::xml_deser::deser_operation_receive_message(response.body().as_ref(), output)
                .map_err(crate::error::ReceiveMessageError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_remove_permission_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::RemovePermissionOutput, crate::error::RemovePermissionError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::RemovePermissionError::unhandled)?;
    Err(crate::error::RemovePermissionError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_remove_permission_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::RemovePermissionOutput, crate::error::RemovePermissionError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::remove_permission_output::Builder::default();
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_send_message_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::SendMessageOutput, crate::error::SendMessageError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::SendMessageError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::SendMessageError::unhandled(generic)),
    };
    Err(match error_code {
        "InvalidMessageContents" => crate::error::SendMessageError {
            meta: generic,
            kind: crate::error::SendMessageErrorKind::InvalidMessageContents({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::invalid_message_contents::Builder::default();
                output =
                    crate::xml_deser::invalid_message_contents(response.body().as_ref(), output)
                        .map_err(crate::error::SendMessageError::unhandled)?;
                output.build()
            }),
        },
        "UnsupportedOperation" => crate::error::SendMessageError {
            meta: generic,
            kind: crate::error::SendMessageErrorKind::UnsupportedOperation({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::unsupported_operation::Builder::default();
                output = crate::xml_deser::unsupported_operation(response.body().as_ref(), output)
                    .map_err(crate::error::SendMessageError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::SendMessageError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_send_message_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::SendMessageOutput, crate::error::SendMessageError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::send_message_output::Builder::default();
        output = crate::xml_deser::deser_operation_send_message(response.body().as_ref(), output)
            .map_err(crate::error::SendMessageError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_send_message_batch_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::SendMessageBatchOutput, crate::error::SendMessageBatchError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::SendMessageBatchError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::SendMessageBatchError::unhandled(generic)),
    };
    Err(match error_code {
        "BatchEntryIdsNotDistinct" => crate::error::SendMessageBatchError {
            meta: generic,
            kind: crate::error::SendMessageBatchErrorKind::BatchEntryIdsNotDistinct({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::batch_entry_ids_not_distinct::Builder::default();
                output = crate::xml_deser::batch_entry_ids_not_distinct(
                    response.body().as_ref(),
                    output,
                )
                .map_err(crate::error::SendMessageBatchError::unhandled)?;
                output.build()
            }),
        },
        "BatchRequestTooLong" => crate::error::SendMessageBatchError {
            meta: generic,
            kind: crate::error::SendMessageBatchErrorKind::BatchRequestTooLong({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::batch_request_too_long::Builder::default();
                output = crate::xml_deser::batch_request_too_long(response.body().as_ref(), output)
                    .map_err(crate::error::SendMessageBatchError::unhandled)?;
                output.build()
            }),
        },
        "EmptyBatchRequest" => crate::error::SendMessageBatchError {
            meta: generic,
            kind: crate::error::SendMessageBatchErrorKind::EmptyBatchRequest({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::empty_batch_request::Builder::default();
                output = crate::xml_deser::empty_batch_request(response.body().as_ref(), output)
                    .map_err(crate::error::SendMessageBatchError::unhandled)?;
                output.build()
            }),
        },
        "InvalidBatchEntryId" => crate::error::SendMessageBatchError {
            meta: generic,
            kind: crate::error::SendMessageBatchErrorKind::InvalidBatchEntryId({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::invalid_batch_entry_id::Builder::default();
                output = crate::xml_deser::invalid_batch_entry_id(response.body().as_ref(), output)
                    .map_err(crate::error::SendMessageBatchError::unhandled)?;
                output.build()
            }),
        },
        "TooManyEntriesInBatchRequest" => crate::error::SendMessageBatchError {
            meta: generic,
            kind: crate::error::SendMessageBatchErrorKind::TooManyEntriesInBatchRequest({
                let _ = response;
                #[allow(unused_mut)]
                let mut output =
                    crate::error::too_many_entries_in_batch_request::Builder::default();
                output = crate::xml_deser::too_many_entries_in_batch_request(
                    response.body().as_ref(),
                    output,
                )
                .map_err(crate::error::SendMessageBatchError::unhandled)?;
                output.build()
            }),
        },
        "UnsupportedOperation" => crate::error::SendMessageBatchError {
            meta: generic,
            kind: crate::error::SendMessageBatchErrorKind::UnsupportedOperation({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::unsupported_operation::Builder::default();
                output = crate::xml_deser::unsupported_operation(response.body().as_ref(), output)
                    .map_err(crate::error::SendMessageBatchError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::SendMessageBatchError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_send_message_batch_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::SendMessageBatchOutput, crate::error::SendMessageBatchError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::send_message_batch_output::Builder::default();
        output =
            crate::xml_deser::deser_operation_send_message_batch(response.body().as_ref(), output)
                .map_err(crate::error::SendMessageBatchError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_set_queue_attributes_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::SetQueueAttributesOutput, crate::error::SetQueueAttributesError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::SetQueueAttributesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::SetQueueAttributesError::unhandled(generic)),
    };
    Err(match error_code {
        "InvalidAttributeName" => crate::error::SetQueueAttributesError {
            meta: generic,
            kind: crate::error::SetQueueAttributesErrorKind::InvalidAttributeName({
                let _ = response;
                #[allow(unused_mut)]
                let mut output = crate::error::invalid_attribute_name::Builder::default();
                output = crate::xml_deser::invalid_attribute_name(response.body().as_ref(), output)
                    .map_err(crate::error::SetQueueAttributesError::unhandled)?;
                output.build()
            }),
        },
        _ => crate::error::SetQueueAttributesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_set_queue_attributes_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::SetQueueAttributesOutput, crate::error::SetQueueAttributesError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::set_queue_attributes_output::Builder::default();
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_tag_queue_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::TagQueueOutput, crate::error::TagQueueError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::TagQueueError::unhandled)?;
    Err(crate::error::TagQueueError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_tag_queue_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::TagQueueOutput, crate::error::TagQueueError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::tag_queue_output::Builder::default();
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_untag_queue_error(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::UntagQueueOutput, crate::error::UntagQueueError> {
    let generic = crate::xml_deser::parse_generic_error(&response)
        .map_err(crate::error::UntagQueueError::unhandled)?;
    Err(crate::error::UntagQueueError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_untag_queue_response(
    response: &http::Response<bytes::Bytes>,
) -> Result<crate::output::UntagQueueOutput, crate::error::UntagQueueError> {
    Ok({
        let _ = response;
        #[allow(unused_mut)]
        let mut output = crate::output::untag_queue_output::Builder::default();
        output.build()
    })
}
