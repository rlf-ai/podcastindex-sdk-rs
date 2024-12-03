# RecentData200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<[**models::Status**](status.md)> |  | [optional]
**feed_count** | Option<**i32**> | Number of items in the `feeds` returned in request  | [optional]
**item_count** | Option<**i32**> | Number of items in the `items` returned in request  | [optional]
**max** | Option<**i32**> | Value of `max` parameter passed to request.  | [optional]
**since** | Option<**i32**> | Value of `since` parameter passed to request.  | [optional]
**description** | Option<**String**> | Description of the response  | [optional]
**next_since** | Option<**i32**> | Value to pass as `since` parameter to get next batch of data  | [optional]
**data** | Option<[**models::Data**](data.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


