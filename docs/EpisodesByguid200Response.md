# EpisodesByguid200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<[**models::Status**](status.md)> |  | [optional]
**id** | Option<**String**> | Value passed to request in the `feedid` parameter  | [optional]
**url** | Option<**String**> | Value passed to request in the `feedurl` parameter. If no `feedurl` passed, value will be null.  | [optional]
**podcast_guid** | Option<**String**> | The GUID from the `podcast:guid` tag in the feed. This value is a unique, global identifier for the podcast.   See the namespace spec for [guid](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid) for details.  | [optional]
**guid** | Option<**String**> | Value passed to request in the `guid` parameter.  | [optional]
**episode** | Option<[**models::EpisodeObject**](episode_object.md)> |  | [optional]
**description** | Option<**String**> | Description of the response  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


