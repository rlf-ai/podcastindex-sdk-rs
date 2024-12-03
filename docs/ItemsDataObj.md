# ItemsDataObj

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**episode_id** | Option<**i32**> | The internal PodcastIndex.org episode ID.  | [optional]
**episode_title** | Option<**String**> | Name of the feed  | [optional]
**episode_description** | Option<**String**> | The item-level description of the episode.   Uses the longer of the possible fields in the feed: `<description>`, `<itunes:summary>` and `<content:encoded>`  | [optional]
**episode_image** | Option<**String**> | The item-level image for the episode  | [optional]
**episode_timestamp** | Option<**i32**> | The date and time the episode was published  | [optional]
**episode_added** | Option<**i32**> | The time the episode was added to the index  | [optional]
**episode_enclosure_url** | Option<**String**> | URL/link to the episode file  | [optional]
**episode_enclosure_length** | Option<**i32**> | The length of the item specified by the `enclosureUrl` in bytes  | [optional]
**episode_enclosure_type** | Option<**String**> | The Content-Type for the item specified by the `enclosureUrl`  | [optional]
**episode_duration** | Option<**i32**> | The estimated length of the item specified by the `enclosureUrl` in seconds. Will be null for `liveItem`.  | [optional]
**episode_type** | Option<[**models::EpisodeType**](episodeType.md)> |  | [optional]
**feed_id** | Option<**i32**> | The internal PodcastIndex.org Feed ID.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


