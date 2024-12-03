# EpisodeObject

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
**duration** | Option<**i32**> | The estimated length of the item specified by the `enclosureUrl` in seconds. Will be null for `liveItem`.  | [optional]
**explicit** | Option<[**models::ExplicitEpisode**](explicit_episode.md)> |  | [optional]
**episode** | Option<**i32**> | Episode number  | [optional]
**episode_type** | Option<[**models::EpisodeType**](episodeType.md)> |  | [optional]
**season** | Option<**i32**> | Season number. May be null for `liveItem`.  | [optional]
**image** | Option<**String**> | The item-level image for the episode  | [optional]
**image_url_hash** | Option<**i32**> | A CRC32 hash of the `image` URL with the protocol (`http://`, `https://`) removed.  | [optional]
**feed_itunes_id** | Option<**i32**> | The iTunes ID of this feed if there is one, and we know what it is.  | [optional]
**feed_image** | Option<**String**> | The channel-level image element.  | [optional]
**feed_image_url_hash** | Option<**i32**> | A CRC32 hash of the `feedImage` URL with the protocol (`http://`, `https://`) removed.  | [optional]
**feed_id** | Option<**i32**> | The internal PodcastIndex.org Feed ID.  | [optional]
**feed_title** | Option<**String**> | Name of the feed  | [optional]
**feed_language** | Option<**String**> | The channel-level language specification of the feed.  Languages accord with the [RSS Language Spec](https://www.rssboard.org/rss-language-codes).  | [optional]
**chapters_url** | Option<**String**> | Link to the JSON file containing the episode chapters  | [optional]
**transcripts** | Option<[**Vec<models::Transcript>**](transcript.md)> | List of transcripts for the episode. May not be reported.  | [optional]
**persons** | Option<[**Vec<models::Person>**](person.md)> | List of people with an interest in this episode. May not be reported.   See the [podcast namespace spec](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#person) for more information.  | [optional]
**social_interact** | Option<[**Vec<models::SocialInteractItem>**](socialInteract_item.md)> | List the social interact data found in the podcast feed. May not be reported.   See the [podcast namespace spec](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#social-interact) for more information.  | [optional]
**value** | Option<[**models::Value**](value.md)> |  | [optional]
**soundbite** | Option<[**models::Soundbite**](soundbite.md)> |  | [optional]
**soundbites** | Option<[**Vec<models::Soundbite>**](soundbite.md)> | Soundbites for episode. May not be reported.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


