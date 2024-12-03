# ValueBatchByepisodeguid200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<[**models::Status**](status.md)> |  | [optional]
**query** | Option<[**models::QueryGuids**](query_guids.md)> |  | [optional]
**value** | Option<[**Vec<models::ValueByepisodeguidBatch>**](value_byepisodeguid_batch.md)> | List of value blocks  | [optional]
**description** | Option<**String**> | Description of the response  | [optional]
**all_found** | Option<**bool**> | Indicates if data was found for all `podcastguid` and `episodeguid` values.  | [optional]
**found** | Option<**i32**> | Number of value data returned.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


