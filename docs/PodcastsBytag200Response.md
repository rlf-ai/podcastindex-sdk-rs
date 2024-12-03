# PodcastsBytag200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<[**models::Status**](status.md)> |  | [optional]
**feeds** | Option<[**Vec<models::FeedBytag>**](feed_bytag.md)> | List of feeds matching request  | [optional]
**count** | Option<**i32**> | Number of items returned in request  | [optional]
**total** | Option<**i32**> | Total number of feeds returnable by endpoint  | [optional]
**next_start_at** | Option<**i32**> | Feed ID to pass to next `start_at` to get next batch of feeds   Only returned when `start_at` passed to request  | [optional]
**description** | Option<**String**> | Description of the response  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


