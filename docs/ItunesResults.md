# ItunesResults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**artist_name** | Option<**String**> | The channel-level author element.   Usually iTunes specific, but could be from another namespace if not present.  | [optional]
**artwork_url100** | Option<**String**> | A URL for the artwork associated with the returned media type.   Note: Apple returns the image sized to value in the field name but the PodcastIndex returns the original image specified in the feed.  | [optional]
**artwork_url30** | Option<**String**> | A URL for the artwork associated with the returned media type.   Note: Apple returns the image sized to value in the field name but the PodcastIndex returns the original image specified in the feed.  | [optional]
**artwork_url60** | Option<**String**> | A URL for the artwork associated with the returned media type.   Note: Apple returns the image sized to value in the field name but the PodcastIndex returns the original image specified in the feed.  | [optional]
**artwork_url600** | Option<**String**> | A URL for the artwork associated with the returned media type.   Note: Apple returns the image sized to value in the field name but the PodcastIndex returns the original image specified in the feed.  | [optional]
**collection_censored_name** | Option<**String**> | The name of the feed.   Note: Apple censors the name but PodcastIndex does not.  | [optional]
**collection_explicitness** | Option<**String**> | Indicates if the feed is marked explicit.  | [optional]
**collection_hd_price** | Option<**i32**> | Price of content. Will always return 0.  | [optional]
**collection_id** | Option<**i32**> | The iTunes Feed ID  | [optional]
**collection_name** | Option<**String**> | Name of the feed  | [optional]
**collection_price** | Option<**i32**> | Price of content. Will always return 0.  | [optional]
**collection_view_url** | Option<**String**> | The URL for viewing the feed on the Apple website.  | [optional]
**content_advisory_rating** | Option<**String**> | Indicates if the feed is explicit or clean.  | [optional]
**country** | Option<**String**> | The country the feed is from.   Note: Right now, always returns \"USA\"  | [optional]
**currency** | Option<**String**> | Currency `*Price` value is in.   Note: will always return \"USD\"  | [optional]
**feed_url** | Option<**String**> | Current feed URL  | [optional]
**genre_ids** | Option<**Vec<i32>**> | List of ids representing the names in the `genres`.   Values are determined by the IDs used in the url of genres on https://podcasts.apple.com/us/genre/podcasts/id26  | [optional]
**genres** | Option<**Vec<String>**> | List of genre names.  | [optional]
**kind** | Option<**String**> | The kind of content returned by the search request.   Note: will always return \"podcast\"  | [optional]
**primary_genre_name** | Option<**String**> | The primary genre name.  | [optional]
**release_date** | Option<**String**> | Date and time of request  | [optional]
**track_censored_name** | Option<**String**> | The name of the feed.   Note: Apple censors the name but PodcastIndex does not.  | [optional]
**track_count** | Option<**i32**> | Number of episodes in feed  | [optional]
**track_explicitness** | Option<**String**> | Indicates if the episode is marked explicit.  | [optional]
**track_hd_price** | Option<**i32**> | Price of content. Will always return 0.  | [optional]
**track_hd_rental_price** | Option<**i32**> | Price of content. Will always return 0.  | [optional]
**track_id** | Option<**i32**> | The iTunes Feed ID  | [optional]
**track_name** | Option<**String**> | Name of the feed  | [optional]
**track_price** | Option<**i32**> | Price of content. Will always return 0.  | [optional]
**track_rental_price** | Option<**i32**> | Price of content. Will always return 0.  | [optional]
**track_view_url** | Option<**String**> | The URL for viewing the feed on the Apple website.  | [optional]
**wrapper_type** | Option<**String**> | The name of the object returned by the search request.   Note: will always return \"track\"  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


