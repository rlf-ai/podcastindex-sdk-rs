# ItemPodcastRandom

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The internal PodcastIndex.org episode ID.  | [optional]
**title** | Option<**String**> | Name of the feed  | [optional]
**link** | Option<**String**> | The channel-level link in the feed  | [optional]
**description** | Option<**String**> | The item-level description of the episode.   Uses the longer of the possible fields in the feed: `<description>`, `<itunes:summary>` and `<content:encoded>`  | [optional]
**guid** | Option<**String**> | The unique identifier for the episode  | [optional]
**date_published** | Option<**i32**> | The date and time the episode was published  | [optional]
**date_published_pretty** | Option<**String**> | The date and time the episode was published formatted as a human readable string.   Note: uses the PodcastIndex server local time to do conversion.  | [optional]
**date_crawled** | Option<**i32**> | The time this episode was found in the feed  | [optional]
**enclosure_url** | Option<**String**> | URL/link to the episode file  | [optional]
**enclosure_type** | Option<**String**> | The Content-Type for the item specified by the `enclosureUrl`  | [optional]
**enclosure_length** | Option<**i32**> | The length of the item specified by the `enclosureUrl` in bytes  | [optional]
**explicit** | Option<[**models::ExplicitEpisode**](explicit_episode.md)> |  | [optional]
**episode** | Option<**i32**> | Episode number  | [optional]
**episode_type** | Option<[**models::EpisodeType**](episodeType.md)> |  | [optional]
**season** | Option<**i32**> | Season number. May be null for `liveItem`.  | [optional]
**image** | Option<**String**> | The item-level image for the episode  | [optional]
**feed_itunes_id** | Option<**i32**> | The iTunes ID of this feed if there is one, and we know what it is.  | [optional]
**feed_image** | Option<**String**> | The channel-level image element.  | [optional]
**feed_id** | Option<**i32**> | The internal PodcastIndex.org Feed ID.  | [optional]
**feed_title** | Option<**String**> | Name of the feed  | [optional]
**feed_language** | Option<**String**> | The channel-level language specification of the feed.  Languages accord with the [RSS Language Spec](https://www.rssboard.org/rss-language-codes).  | [optional]
**categories** | Option<[**serde_json::Value**](.md)> | An array of categories, where the index is the Category ID and the value is the Category Name.   All Category numbers and names are returned by the `categories/list` endpoint.  | [optional]
**chapters_url** | Option<**String**> | Link to the JSON file containing the episode chapters  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


