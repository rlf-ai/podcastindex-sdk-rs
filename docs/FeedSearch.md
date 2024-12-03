# FeedSearch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The internal PodcastIndex.org Feed ID.  | [optional]
**podcast_guid** | Option<**String**> | The GUID from the `podcast:guid` tag in the feed. This value is a unique, global identifier for the podcast.   See the namespace spec for [guid](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid) for details.  | [optional]
**title** | Option<**String**> | Name of the feed  | [optional]
**url** | Option<**String**> | Current feed URL  | [optional]
**original_url** | Option<**String**> | The URL of the feed, before it changed to the current `url` value.  | [optional]
**link** | Option<**String**> | The channel-level link in the feed  | [optional]
**description** | Option<**String**> | The channel-level description   Uses the longer of the possible fields in the feed: `<description>`, `<itunes:summary>` and `<content:encoded>`  | [optional]
**author** | Option<**String**> | The channel-level author element.   Usually iTunes specific, but could be from another namespace if not present.  | [optional]
**owner_name** | Option<**String**> | The channel-level owner:name element.   Usually iTunes specific, but could be from another namespace if not present.  | [optional]
**image** | Option<**String**> | The channel-level image element.  | [optional]
**artwork** | Option<**String**> | The seemingly best artwork we can find for the feed.  Might be the same as `image` in most instances.  | [optional]
**last_update_time** | Option<**i32**> | The channel-level pubDate for the feed, if it’s sane.  If not, this is a heuristic value, arrived at by analyzing other parts of the feed, like item-level pubDates.  | [optional]
**last_crawl_time** | Option<**i32**> | The last time we attempted to pull this feed from its url.  | [optional]
**last_parse_time** | Option<**i32**> | The last time we tried to parse the downloaded feed content.  | [optional]
**last_good_http_status_time** | Option<**i32**> | Timestamp of the last time we got a \"good\", meaning non-4xx/non-5xx, status code when pulling this feed from its url.  | [optional]
**last_http_status** | Option<**i32**> | The last http status code we got when pulling this feed from its url.   You will see some made up status codes sometimes. These are what we use to track state within the feed puller. These all start with 9xx.  | [optional]
**content_type** | Option<**String**> | The Content-Type header from the last time we pulled this feed from its url.  | [optional]
**itunes_id** | Option<**i32**> | The iTunes ID of this feed if there is one, and we know what it is.  | [optional]
**generator** | Option<**String**> | The channel-level generator element if there is one.  | [optional]
**language** | Option<**String**> | The channel-level language specification of the feed.  Languages accord with the [RSS Language Spec](https://www.rssboard.org/rss-language-codes).  | [optional]
**explicit** | Option<**bool**> | Is feed marked as explicit  | [optional]
**r#type** | Option<[**models::Type**](type.md)> |  | [optional]
**medium** | Option<**String**> | The value of the `podcast:medium` attribute for the feed.   See the [medium](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#medium) description in the Podcast Namespace for more information.  | [optional]
**dead** | Option<**i32**> | At some point, we give up trying to process a feed and mark it as dead. This is usually after 1000 errors without a successful pull/parse cycle. Once the feed is marked dead, we only check it once per month.  | [optional]
**episode_count** | Option<**i32**> | Number of episodes for this feed known to the index.  | [optional]
**crawl_errors** | Option<**i32**> | The number of errors we’ve encountered trying to pull a copy of the feed. Errors are things like a 500 or 404 response, a server timeout, bad encoding, etc.  | [optional]
**parse_errors** | Option<**i32**> | The number of errors we’ve encountered trying to parse the feed content. Errors here are things like not well-formed xml, bad character encoding, etc.   We fix many of these types of issues on the fly when parsing. We only increment the errors count when we can’t fix it.  | [optional]
**categories** | Option<[**serde_json::Value**](.md)> | An array of categories, where the index is the Category ID and the value is the Category Name.   All Category numbers and names are returned by the `categories/list` endpoint.  | [optional]
**locked** | Option<[**models::Locked**](locked.md)> |  | [optional]
**image_url_hash** | Option<**i32**> | A CRC32 hash of the `image` URL with the protocol (`http://`, `https://`) removed.  | [optional]
**newest_item_pubdate** | Option<**i32**> | The time the most recent episode in the feed was published.   Note: some endpoints use `newestItemPubdate` while others use `newestItemPublishTime`.  They return the same information. See https://github.com/Podcastindex-org/api/issues/3 to track when the property name is updated.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


