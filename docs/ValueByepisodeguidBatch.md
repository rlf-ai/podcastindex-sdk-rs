# ValueByepisodeguidBatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**podcast_guid** | Option<**String**> | The GUID from the `podcast:guid` tag in the feed. This value is a unique, global identifier for the podcast.   See the namespace spec for [guid](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid) for details.  | [optional]
**guid** | Option<**String**> | The unique identifier for the episode  | [optional]
**title** | Option<**String**> | Name of the episode  | [optional]
**feed_title** | Option<**String**> | Name of the feed  | [optional]
**model** | Option<[**models::ModelV4v**](model_v4v.md)> |  | [optional]
**destinations** | Option<[**Vec<models::DestinationV4v>**](destination_v4v.md)> | List of destinations where \"Value for Value\" payments should be sent.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


