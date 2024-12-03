/*
 * PodcastIndex.org API
 *
 * Download the openapi spec in [json](pi_api.json) or [yaml](pi_api.yaml) format.  # Overview  The Podcast Index (Podcast Index LLC) is a software developer focused partnership that provides tools and data to anyone who aspires to create new and exciting Podcast experiences without the heavy lifting of indexing, aggregation and data management.  # Example Code  Users have provided example code for working the API in the following languages:    - [AWS Lambda - python](https://github.com/tbowers/python-podcastindex-org-lambda)   - [Bash](https://github.com/suorcd/Bash-podcastindex-org-example)   - [C#](https://github.com/ComicStrip/csharp-podcastindex-org-example)   - [Elisp](https://github.com/sabexx/elisp-example)   - [Flutter/Dart](https://github.com/crediblecreative/flutter-dart-podcastindex-org-example)   - [Go](https://github.com/ComicStrip/Go-podcastindex-org-example)   - [Go](https://github.com/kilobit/podcast-index-client)   - [Java](https://github.com/ComicStrip/Java-podcastindex-org-example)   - [Java](https://github.com/stucoates/PodcastIndexJavaClient)   - [Node.js](https://github.com/ComicStrip/node.js-podcastindex-org-example)   - [PHP](https://github.com/Podcastindex-org/example-code)   - [Python](https://github.com/tbowers/python-podcastindex-org-example)   - [Swift](https://github.com/ComicStrip/Swift-podcastindex-org-example)   - ... [More](https://github.com/Podcastindex-org-Examples)  Don't see your desired language, create an example repo and create a [Pull Request](https://github.com/Podcastindex-org/docs-api/pulls) with a link to your example code project!  # Libraries  User created libraries for working with the API:    - Java       - [podcast4j](https://github.com/yusufyilmazfr/podcast4j)   - .NET       - [PodcastIndexSharp](https://www.nuget.org/packages/PodcastIndexSharp)   - Node.js/npm/yarn       - [podcast-index-api](https://www.npmjs.com/package/podcast-index-api)       - [podcastdx-client](https://www.npmjs.com/package/podcastdx-client)   - PHP       - [podcastindex-php](https://github.com/LowSociety/podcastindex-php)   - Python/pip       - [python-podcastindex](https://pypi.org/project/python-podcastindex/)   - R       - [podindexr](https://github.com/rpodcast/podindexr)   - Ruby       - [podcast-index](https://github.com/jasonyork/podcast-index)   - Swift       - [PodcastIndexKit](https://github.com/SparrowTek/PodcastIndexKit)   - Kotlin       - [PodcastIndex-SDK](https://github.com/mr3y-the-programmer/PodcastIndex-SDK)  Are we missing a library? Did you create one for a different language? Create a [Pull Request](https://github.com/Podcastindex-org/docs-api/pulls) with a link to the new library!  # Postman  A collection file for use in the [Postman](https://www.postman.com/) application is available for this API.    1. Download the contents of the [Postman Docs](https://github.com/Podcastindex-org/docs-api/tree/master/Postman%20Docs) folder.   2. Import the `PodcastIndex.postman_collection.json` collection to Postman   3. Import the `PodcastIndexOrgEnvironment.postman_environment.json` to Postman   4. Click \"Environments\" on the left sidebar   5. Select the checkbox next to the PodcastIndexOrgEnvironment entry   6. Set `AuthKey` and `SeceretKey` values under the \"Current Value\" column using your API information    7. Click \"Collections\" from the sidebar   8. Select PodcastIndex   9. Select and run the endpoint to test  # Contributing  The source for this API documentation is available at [https://github.com/Podcastindex-org/docs-api](https://github.com/Podcastindex-org/docs-api). Submit an Issue or create a Pull Request.  # Authentication Details  Sending an API request is easy. We use an Amazon-style request authorization token to secure each request.   Register for a free API key at https://api.podcastindex.org/   These headers parameters are required for each request: `User-Agent`, `X-Auth-Date`, `X-Auth-Key`, `Authorization`   See [Authentication](#auth) for description of parameters.  # Legal  Legal    - [Privacy Policy](https://github.com/Podcastindex-org/legal/blob/main/PrivacyPolicy.md)   - [Terms of Service](https://github.com/Podcastindex-org/legal/blob/main/TermsOfService.md)   - [License](https://github.com/Podcastindex-org/docs-api/blob/master/LICENSE)
 *
 * The version of the OpenAPI document: 1.12.1
 * Contact: info@podcastindex.org
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// FeedPodcast : Known details of podcast feed
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedPodcast {
    /// The internal PodcastIndex.org Feed ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The GUID from the `podcast:guid` tag in the feed. This value is a unique, global identifier for the podcast.   See the namespace spec for [guid](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#guid) for details.
    #[serde(rename = "podcastGuid", skip_serializing_if = "Option::is_none")]
    pub podcast_guid: Option<String>,
    /// Name of the feed
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Current feed URL
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The URL of the feed, before it changed to the current `url` value.
    #[serde(rename = "originalUrl", skip_serializing_if = "Option::is_none")]
    pub original_url: Option<String>,
    /// The channel-level link in the feed
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// The channel-level description   Uses the longer of the possible fields in the feed: `<description>`, `<itunes:summary>` and `<content:encoded>`
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The channel-level author element.   Usually iTunes specific, but could be from another namespace if not present.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// The channel-level owner:name element.   Usually iTunes specific, but could be from another namespace if not present.
    #[serde(rename = "ownerName", skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    /// The channel-level image element.
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// The seemingly best artwork we can find for the feed.  Might be the same as `image` in most instances.
    #[serde(rename = "artwork", skip_serializing_if = "Option::is_none")]
    pub artwork: Option<String>,
    /// The channel-level pubDate for the feed, if it’s sane.  If not, this is a heuristic value, arrived at by analyzing other parts of the feed, like item-level pubDates.
    #[serde(rename = "lastUpdateTime", skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<i32>,
    /// The last time we attempted to pull this feed from its url.
    #[serde(rename = "lastCrawlTime", skip_serializing_if = "Option::is_none")]
    pub last_crawl_time: Option<i32>,
    /// The last time we tried to parse the downloaded feed content.
    #[serde(rename = "lastParseTime", skip_serializing_if = "Option::is_none")]
    pub last_parse_time: Option<i32>,
    /// Timestamp of the last time we got a \"good\", meaning non-4xx/non-5xx, status code when pulling this feed from its url.
    #[serde(
        rename = "lastGoodHttpStatusTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_good_http_status_time: Option<i32>,
    /// The last http status code we got when pulling this feed from its url.   You will see some made up status codes sometimes. These are what we use to track state within the feed puller. These all start with 9xx.
    #[serde(rename = "lastHttpStatus", skip_serializing_if = "Option::is_none")]
    pub last_http_status: Option<i32>,
    /// The Content-Type header from the last time we pulled this feed from its url.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The iTunes ID of this feed if there is one, and we know what it is.
    #[serde(
        rename = "itunesId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub itunes_id: Option<Option<i32>>,
    /// The type as specified by the `itunes:type` in the feed XML.
    #[serde(rename = "itunesType", skip_serializing_if = "Option::is_none")]
    pub itunes_type: Option<String>,
    /// The channel-level generator element if there is one.
    #[serde(rename = "generator", skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
    /// The channel-level language specification of the feed.  Languages accord with the [RSS Language Spec](https://www.rssboard.org/rss-language-codes).
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Is feed marked as explicit
    #[serde(rename = "explicit", skip_serializing_if = "Option::is_none")]
    pub explicit: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::Type>,
    /// The value of the `podcast:medium` attribute for the feed.   See the [medium](https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md#medium) description in the Podcast Namespace for more information.
    #[serde(rename = "medium", skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    /// At some point, we give up trying to process a feed and mark it as dead. This is usually after 1000 errors without a successful pull/parse cycle. Once the feed is marked dead, we only check it once per month.
    #[serde(rename = "dead", skip_serializing_if = "Option::is_none")]
    pub dead: Option<i32>,
    /// The md5 hash of the following feed items in hex format.    - `title`   - `link`   - `feedLanguage`   - `generator`   - `author`   - `ownerName`   - `ownerEmail` (note: not exposed via the API)  Pseudo-code:        chash = md5(title+link+feedLanguage+generator+author+ownerName+ownerEmail)
    #[serde(rename = "chash", skip_serializing_if = "Option::is_none")]
    pub chash: Option<String>,
    /// Number of episodes for this feed known to the index.
    #[serde(rename = "episodeCount", skip_serializing_if = "Option::is_none")]
    pub episode_count: Option<i32>,
    /// The number of errors we’ve encountered trying to pull a copy of the feed. Errors are things like a 500 or 404 response, a server timeout, bad encoding, etc.
    #[serde(rename = "crawlErrors", skip_serializing_if = "Option::is_none")]
    pub crawl_errors: Option<i32>,
    /// The number of errors we’ve encountered trying to parse the feed content. Errors here are things like not well-formed xml, bad character encoding, etc.   We fix many of these types of issues on the fly when parsing. We only increment the errors count when we can’t fix it.
    #[serde(rename = "parseErrors", skip_serializing_if = "Option::is_none")]
    pub parse_errors: Option<i32>,
    /// An array of categories, where the index is the Category ID and the value is the Category Name.   All Category numbers and names are returned by the `categories/list` endpoint.
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<serde_json::Value>,
    #[serde(rename = "locked", skip_serializing_if = "Option::is_none")]
    pub locked: Option<models::Locked>,
    /// A CRC32 hash of the `image` URL with the protocol (`http://`, `https://`) removed.
    #[serde(rename = "imageUrlHash", skip_serializing_if = "Option::is_none")]
    pub image_url_hash: Option<i32>,
    #[serde(
        rename = "value",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<Option<Box<models::Value>>>,
    #[serde(
        rename = "funding",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub funding: Option<Option<Box<models::Funding>>>,
}

impl FeedPodcast {
    /// Known details of podcast feed
    pub fn new() -> FeedPodcast {
        FeedPodcast {
            id: None,
            podcast_guid: None,
            title: None,
            url: None,
            original_url: None,
            link: None,
            description: None,
            author: None,
            owner_name: None,
            image: None,
            artwork: None,
            last_update_time: None,
            last_crawl_time: None,
            last_parse_time: None,
            last_good_http_status_time: None,
            last_http_status: None,
            content_type: None,
            itunes_id: None,
            itunes_type: None,
            generator: None,
            language: None,
            explicit: None,
            r#type: None,
            medium: None,
            dead: None,
            chash: None,
            episode_count: None,
            crawl_errors: None,
            parse_errors: None,
            categories: None,
            locked: None,
            image_url_hash: None,
            value: None,
            funding: None,
        }
    }
}