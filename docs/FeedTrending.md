# FeedTrending

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The internal PodcastIndex.org Feed ID.  | [optional]
**url** | Option<**String**> | Current feed URL  | [optional]
**title** | Option<**String**> | Name of the feed  | [optional]
**description** | Option<**String**> | The channel-level description   Uses the longer of the possible fields in the feed: `<description>`, `<itunes:summary>` and `<content:encoded>`  | [optional]
**author** | Option<**String**> | The channel-level author element.   Usually iTunes specific, but could be from another namespace if not present.  | [optional]
**image** | Option<**String**> | The channel-level image element.  | [optional]
**artwork** | Option<**String**> | The seemingly best artwork we can find for the feed.  Might be the same as `image` in most instances.  | [optional]
**newest_item_publish_time** | Option<**i32**> | The time the most recent episode in the feed was published.   Note: some endpoints use `newestItemPubdate` while others use `newestItemPublishTime`.  They return the same information. See https://github.com/Podcastindex-org/api/issues/3 to track when the property name is updated.  | [optional]
**itunes_id** | Option<**i32**> | The iTunes ID of this feed if there is one, and we know what it is.  | [optional]
**trend_score** | Option<**i32**> | The ranking for how the podcast is trending in the index  | [optional]
**language** | Option<**String**> | The channel-level language specification of the feed.  Languages accord with the [RSS Language Spec](https://www.rssboard.org/rss-language-codes).  | [optional]
**categories** | Option<[**serde_json::Value**](.md)> | An array of categories, where the index is the Category ID and the value is the Category Name.   All Category numbers and names are returned by the `categories/list` endpoint.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


