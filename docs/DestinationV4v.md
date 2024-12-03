# DestinationV4v

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name for the destination  | [optional]
**address** | Option<**String**> | Address of node to receive payment  | [optional]
**r#type** | Option<[**models::TypeDestination**](type_destination.md)> |  | [optional]
**split** | Option<**i32**> | Share of payment the destination should receive  | [optional]
**fee** | Option<**bool**> | Indicates if destination is included due to a fee being charged. May not be reported.  | [optional]
**custom_key** | Option<**String**> | The name of a custom record key to send along with the payment. May not be reported.   See the [podcast namespace spec](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#value) and [value specification](https://github.com/Podcastindex-org/podcast-namespace/blob/main/value/value.md) for more information.  | [optional]
**custom_value** | Option<**String**> | A custom value to pass along with the payment. This is considered the value that belongs to the customKey. May not be reported.   See the [podcast namespace spec](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#value) and [value specification](https://github.com/Podcastindex-org/podcast-namespace/blob/main/value/value.md) for more information.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


