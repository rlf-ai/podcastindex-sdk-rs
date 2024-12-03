# LiveitemPodcast

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
**start_time** | Option<**i32**> | The time the livestream starts  | [optional]
**end_time** | Option<**i32**> | The time the livestream ends  | [optional]
**status** | Option<[**models::StatusLive**](status_live.md)> |  | [optional]
**content_link** | Option<**String**> | Link TODO  | [optional]
**duration** | Option<**i32**> | The estimated length of the item specified by the `enclosureUrl` in seconds. Will be null for `liveItem`.  | [optional]
**explicit** | Option<[**models::ExplicitEpisode**](explicit_episode.md)> |  | [optional]
**episode** | Option<**i32**> | Episode number  | [optional]
**episode_type** | Option<[**models::EpisodeType**](episodeType.md)> |  | [optional]
**season** | Option<**i32**> | Season number. May be null for `liveItem`.  | [optional]
**image** | Option<**String**> | The item-level image for the episode  | [optional]
**feed_itunes_id** | Option<**i32**> | The iTunes ID of this feed if there is one, and we know what it is.  | [optional]
**feed_image** | Option<**String**> | The channel-level image element.  | [optional]
**feed_id** | Option<**i32**> | The internal PodcastIndex.org Feed ID.  | [optional]
**feed_language** | Option<**String**> | The channel-level language specification of the feed.  Languages accord with the [RSS Language Spec](https://www.rssboard.org/rss-language-codes).  | [optional]
**feed_dead** | Option<**i32**> | At some point, we give up trying to process a feed and mark it as dead. This is usually after 1000 errors without a successful pull/parse cycle. Once the feed is marked dead, we only check it once per month.  | [optional]
**feed_duplicate_of** | Option<**i32**> | The internal PodcastIndex.org Feed ID this feed duplicates. May be null except in `podcasts/dead`.  | [optional]
**chapters_url** | Option<**String**> | Link to the JSON file containing the episode chapters  | [optional]
**transcript_url** | Option<**String**> | Link to the file containing the episode transcript   Note: in most use cases, the `transcripts` value should be used instead  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


