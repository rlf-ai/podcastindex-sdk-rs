# FeedsRecentInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The internal PodcastIndex.org Feed ID.  | [optional]
**url** | Option<**String**> | Current feed URL  | [optional]
**title** | Option<**String**> | Name of the feed  | [optional]
**newest_item_publish_time** | Option<**i32**> | The time the most recent episode in the feed was published.   Note: some endpoints use `newestItemPubdate` while others use `newestItemPublishTime`.  They return the same information. See https://github.com/Podcastindex-org/api/issues/3 to track when the property name is updated.  | [optional]
**oldest_item_publish_time** | Option<**i32**> | The date and time the oldest episode in the feed/index  | [optional]
**itunes_id** | Option<**i32**> | The iTunes ID of this feed if there is one, and we know what it is.  | [optional]
**language** | Option<**String**> | The channel-level language specification of the feed.  Languages accord with the [RSS Language Spec](https://www.rssboard.org/rss-language-codes).  | [optional]
**categories** | Option<[**serde_json::Value**](.md)> | An array of categories, where the index is the Category ID and the value is the Category Name.   All Category numbers and names are returned by the `categories/list` endpoint.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


