# FeedsDataObj

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**feed_id** | Option<**i32**> | The internal PodcastIndex.org Feed ID.  | [optional]
**feed_url** | Option<**String**> | Current feed URL  | [optional]
**feed_title** | Option<**String**> | Name of the feed  | [optional]
**feed_description** | Option<**String**> | The channel-level description   Uses the longer of the possible fields in the feed: `<description>`, `<itunes:summary>` and `<content:encoded>`  | [optional]
**feed_image** | Option<**String**> | The channel-level image element.  | [optional]
**feed_language** | Option<**String**> | The channel-level language specification of the feed.  Languages accord with the [RSS Language Spec](https://www.rssboard.org/rss-language-codes).  | [optional]
**feed_itunes_id** | Option<**i32**> | The iTunes ID of this feed if there is one, and we know what it is.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


