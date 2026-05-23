## [Introduction](https://www.pexels.com/api/documentation#introduction)

The Pexels API enables programmatic access to the full Pexels content library, including photos, videos. All content is available free of charge, and you are welcome to use Pexels content for anything you'd like, as long as it is within our [Title](https://www.pexels.com/api/documentation#guidelines).

The Pexels API is a RESTful JSON API, and you can interact with it from any language or framework with a HTTP library. Alternately, Pexels maintains some official [Title](https://www.pexels.com/api/documentation#client_libraries) you can use.

If you have any questions, please visit our [Help Center](https://help.pexels.com/hc/en-us/categories/900001326143-API) for answers and troubleshooting.

**Note:** Video endpoints are now available at `https://api.pexels.com/v1/videos/`. The `https://api.pexels.com/videos/` endpoints will be deprecated in the future, please update your code to use the new path.

## [Guidelines](https://www.pexels.com/api/documentation#guidelines)

Whenever you are doing an API request make sure to show a **prominent link to Pexels**. You can use a text link (e.g. "Photos provided by Pexels") or a link with our logo.

Always credit our photographers when possible (e.g. "Photo by John Doe on Pexels" with a link to the photo page on Pexels).

You may not copy or replicate core functionality of Pexels (including making Pexels content available as a wallpaper app).

Do not abuse the API. By default, the API is rate-limited to 200 requests per hour and 20,000 requests per month. [You may contact us to request a higher limit](mailto:api@pexels.com), but please include examples, or be prepared to give a demo, that clearly shows your use of the API with attribution. If you meet our API terms, you can get unlimited requests for free.

Abuse of the Pexels API, including but not limited to attempting to work around the rate limit, will lead to termination of your API access.

Linking back to Pexels

| ``` 1 2 3 4 5 6 7 8 9 10 11 12 13 ``` | ``` <a href="https://www.pexels.com">Photos provided by Pexels</a>  <!-- or show our white logo -->  <a href="https://www.pexels.com">   <img src="https://images.pexels.com/lib/api/pexels-white.png" /> </a>  <!-- or show our black logo -->  <a href="https://www.pexels.com">   <img src="https://images.pexels.com/lib/api/pexels.png" /> </a> ``` |
| --- | --- |

Linking back to a Photo

| ``` 1 ``` | ``` This <a href="https://www.pexels.com/photo/food-dinner-lunch-meal-4147875">Photo</a> was taken by <a href="https://www.pexels.com/@daria">Daria</a> on Pexels. ``` |
| --- | --- |

## [Client Libraries](https://www.pexels.com/api/documentation#client_libraries)

Pexels maintains a number of official API client libraries that you can use to interact with the Pexels API:

| Language | Package | Github | Changelog | Version |
| --- | --- | --- | --- | --- |
| Ruby | [rubygems](https://rubygems.org/gems/pexels) | [pexels-ruby](https://github.com/pexels/pexels-ruby) | [changelog](https://github.com/pexels/pexels-ruby/blob/master/CHANGES.md) | 0.3.0 |
| Javascript | [npm](https://www.npmjs.com/package/pexels) | [pexels-javascript](https://github.com/pexels/pexels-javascript) | [changelog](https://github.com/pexels/pexels-javascript/releases) | 1.2.1 |
| .net | [nuget](https://www.nuget.org/packages/PexelsDotNetSDK) | [PexelsDotNetSDK](https://github.com/pexels/PexelsDotNetSDK) | [changelog](https://github.com/pexels/PexelsDotNetSDK/blob/master/Changes.md) | 1.0.6 |

Please read the documentation for the client library you'd like to use for more information about syntax (code samples for each library are available on this documentation). Issues and Pull Requests on Github are also welcome!

If you have created an unofficial Pexels API library for a different language please feel free to let us know about it!

## [Authorization](https://www.pexels.com/api/documentation#authorization)

Authorization is required for the Pexels API. Anyone with a Pexels account can [request an API key](https://www.pexels.com/api), which you will receive instantly.

All requests you make to the API will need to include your key. This is provided by adding an `Authorization` header.

```bash
curl -H "Authorization: YOUR_API_KEY" \
  "https://api.pexels.com/v1/search?query=people"
```

## [Request Statistics](https://www.pexels.com/api/documentation#statistics)

To see how many requests you have left in your monthly quota, successful requests from the Pexels API include three HTTP headers:

| Response Header | Meaning |
| --- | --- |
| `X-Ratelimit-Limit` | Your total request limit for the monthly period |
| `X-Ratelimit-Remaining` | How many of these requests remain |
| `X-Ratelimit-Reset` | UNIX timestamp of when the currently monthly period will roll over |

**Note:** These response headers are only returned on successful (`2xx`) responses. They are not included with other responses, including `429 Too Many Requests`, which indicates you have exceeded your rate limit. Please be sure to keep track of `X-Ratelimit-Remaining` and `X-Ratelimit-Reset` in order to manage your request limit.

Example of Rate Limit Headers

| ``` 1 2 3 ``` | ``` X-Ratelimit-Limit: 20000 X-Ratelimit-Remaining: 19684 X-Ratelimit-Reset: 1590529646 ``` |
| --- | --- |

## [Pagination](https://www.pexels.com/api/documentation#pagination)

Most Pexels API requests return multiple records at once. All of these endpoints are paginated, and can return a maximum of **80** requests at one time. Each paginated request accepts the same parameters and returns the same pagination data in the response.

**Note:** The `prev_page` and `next_page` response attributes will only be returned if there is a corresponding page.

Pagination Request Parameters

| ``` 1 ``` | ``` GET https://api.pexels.com/v1/curated?page=2&per_page=40 ``` |
| --- | --- |

Pagination Response Attributes

| ``` 1 2 3 4 5 6 7 ``` | ``` {   "page": 2,   "per_page": 40,   "total_results": 8000,   "next_page": "https://api.pexels.com/v1/curated?page=3&per_page=40",   "prev_page": "https://api.pexels.com/v1/curated?page=1&per_page=40" } ``` |
| --- | --- |

## [The Photo Resource](https://www.pexels.com/api/documentation#photos-overview)

The `Photo` resource is a JSON formatted version of a Pexels photo. The Photo API endpoints respond with the photo data formatted in this shape.

#### [Response](https://www.pexels.com/api/documentation#photos-overview__response)

The Photo Resource

| ``` 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 ``` | ``` {   "id": 2014422,   "width": 3024,   "height": 3024,   "url": "https://www.pexels.com/photo/brown-rocks-during-golden-hour-2014422/",   "photographer": "Joey Farina",   "photographer_url": "https://www.pexels.com/@joey",   "photographer_id": 680589,   "avg_color": "#978E82",   "src": {     "original": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg",     "large2x": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940",     "large": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&h=650&w=940",     "medium": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&h=350",     "small": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&h=130",     "portrait": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800",     "landscape": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200",     "tiny": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280"   },   "liked": false,   "alt": "Brown Rocks During Golden Hour" } ``` |
| --- | --- |

## [Search for Photos](https://www.pexels.com/api/documentation#photos-search)

### GET https://api.pexels.com/v1/search

This endpoint enables you to search Pexels for any topic that you would like. For example your query could be something broad like `Nature`, `Tigers`, `People`. Or it could be something specific like `Group of people working`.

#### [Parameters](https://www.pexels.com/api/documentation#photos-search__parameters)

#### [Response](https://www.pexels.com/api/documentation#photos-search__response)

```bash
curl -H "Authorization: YOUR_API_KEY" \
  "https://api.pexels.com/v1/search?query=nature&per_page=1"
```

Example Response

| ``` 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 ``` | ``` {   "total_results": 10000,   "page": 1,   "per_page": 1,   "photos": [     {       "id": 3573351,       "width": 3066,       "height": 3968,       "url": "https://www.pexels.com/photo/trees-during-day-3573351/",       "photographer": "Lukas Rodriguez",       "photographer_url": "https://www.pexels.com/@lukas-rodriguez-1845331",       "photographer_id": 1845331,       "avg_color": "#374824",       "src": {         "original": "https://images.pexels.com/photos/3573351/pexels-photo-3573351.png",         "large2x": "https://images.pexels.com/photos/3573351/pexels-photo-3573351.png?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940",         "large": "https://images.pexels.com/photos/3573351/pexels-photo-3573351.png?auto=compress&cs=tinysrgb&h=650&w=940",         "medium": "https://images.pexels.com/photos/3573351/pexels-photo-3573351.png?auto=compress&cs=tinysrgb&h=350",         "small": "https://images.pexels.com/photos/3573351/pexels-photo-3573351.png?auto=compress&cs=tinysrgb&h=130",         "portrait": "https://images.pexels.com/photos/3573351/pexels-photo-3573351.png?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800",         "landscape": "https://images.pexels.com/photos/3573351/pexels-photo-3573351.png?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200",         "tiny": "https://images.pexels.com/photos/3573351/pexels-photo-3573351.png?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280"       },       "liked": false,       "alt": "Brown Rocks During Golden Hour"     }   ],   "next_page": "https://api.pexels.com/v1/search/?page=2&per_page=1&query=nature" } ``` |
| --- | --- |

## [Curated Photos](https://www.pexels.com/api/documentation#photos-curated)

### GET https://api.pexels.com/v1/curated

This endpoint enables you to receive real-time photos curated by the Pexels team.

We add at least one new photo per hour to our curated list so that you always get a changing selection of trending photos.

#### [Parameters](https://www.pexels.com/api/documentation#photos-curated__parameters)

#### [Response](https://www.pexels.com/api/documentation#photos-curated__response)

```bash
curl -H "Authorization: YOUR_API_KEY" \
  "https://api.pexels.com/v1/curated?per_page=1"
```

Example Response

| ``` 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 ``` | ``` {   "page": 1,   "per_page": 1,   "photos": [     {       "id": 2880507,       "width": 4000,       "height": 6000,       "url": "https://www.pexels.com/photo/woman-in-white-long-sleeved-top-and-skirt-standing-on-field-2880507/",       "photographer": "Deden Dicky Ramdhani",       "photographer_url": "https://www.pexels.com/@drdeden88",       "photographer_id": 1378810,       "avg_color": "#7E7F7B",       "src": {         "original": "https://images.pexels.com/photos/2880507/pexels-photo-2880507.jpeg",         "large2x": "https://images.pexels.com/photos/2880507/pexels-photo-2880507.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940",         "large": "https://images.pexels.com/photos/2880507/pexels-photo-2880507.jpeg?auto=compress&cs=tinysrgb&h=650&w=940",         "medium": "https://images.pexels.com/photos/2880507/pexels-photo-2880507.jpeg?auto=compress&cs=tinysrgb&h=350",         "small": "https://images.pexels.com/photos/2880507/pexels-photo-2880507.jpeg?auto=compress&cs=tinysrgb&h=130",         "portrait": "https://images.pexels.com/photos/2880507/pexels-photo-2880507.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800",         "landscape": "https://images.pexels.com/photos/2880507/pexels-photo-2880507.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200",         "tiny": "https://images.pexels.com/photos/2880507/pexels-photo-2880507.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280"       },       "liked": false,       "alt": "Brown Rocks During Golden Hour"     }   ],   "next_page": "https://api.pexels.com/v1/curated/?page=2&per_page=1" } ``` |
| --- | --- |

## [Get a Photo](https://www.pexels.com/api/documentation#photos-show)

### GET https://api.pexels.com/v1/photos/:id

Retrieve a specific `Photo` from its id.

#### [Parameters](https://www.pexels.com/api/documentation#photos-show__parameters)

#### [Response](https://www.pexels.com/api/documentation#photos-show__response)

Returns a `Photo` object

```bash
curl -H "Authorization: YOUR_API_KEY" \
  "https://api.pexels.com/v1/photos/2014422"
```

Example Response

| ``` 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 ``` | ``` {   "id": 2014422,   "width": 3024,   "height": 3024,   "url": "https://www.pexels.com/photo/brown-rocks-during-golden-hour-2014422/",   "photographer": "Joey Farina",   "photographer_url": "https://www.pexels.com/@joey",   "photographer_id": 680589,   "avg_color": "#978E82",   "src": {     "original": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg",     "large2x": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940",     "large": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&h=650&w=940",     "medium": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&h=350",     "small": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&h=130",     "portrait": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800",     "landscape": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200",     "tiny": "https://images.pexels.com/photos/2014422/pexels-photo-2014422.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280"   },   "liked": false,   "alt": "Brown Rocks During Golden Hour" } ``` |
| --- | --- |

## [The Video Resource](https://www.pexels.com/api/documentation#videos-overview)

The `Video` resource is a JSON formatted version of a Pexels video. The Video API endpoints respond with the video data formatted in this shape.

#### [Response](https://www.pexels.com/api/documentation#videos-overview__response)

The Video Resource

| ``` 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 ``` | ``` {   "id": 2499611,   "width": 1080,   "height": 1920,   "url": "https://www.pexels.com/video/2499611/",   "image": "https://images.pexels.com/videos/2499611/free-video-2499611.jpg?fit=crop&w=1200&h=630&auto=compress&cs=tinysrgb",   "full_res": null,   "tags": [],   "duration": 22,   "user": {     "id": 680589,     "name": "Joey Farina",     "url": "https://www.pexels.com/@joey"   },   "video_files": [     {       "id": 125004,       "quality": "hd",       "file_type": "video/mp4",       "width": 1080,       "height": 1920,       "fps": 23.976,       "link": "https://player.vimeo.com/external/342571552.hd.mp4?s=6aa6f164de3812abadff3dde86d19f7a074a8a66&profile_id=175&oauth2_token_id=57447761"     },     {       "id": 125005,       "quality": "sd",       "file_type": "video/mp4",       "width": 540,       "height": 960,       "fps": 23.976,       "link": "https://player.vimeo.com/external/342571552.sd.mp4?s=e0df43853c25598dfd0ec4d3f413bce1e002deef&profile_id=165&oauth2_token_id=57447761"     },     {       "id": 125006,       "quality": "sd",       "file_type": "video/mp4",       "width": 240,       "height": 426,       "fps": 23.976,       "link": "https://player.vimeo.com/external/342571552.sd.mp4?s=e0df43853c25598dfd0ec4d3f413bce1e002deef&profile_id=139&oauth2_token_id=57447761"     }     ...   ],   "video_pictures": [     {       "id": 308178,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-0.jpg",       "nr": 0     },     {       "id": 308179,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-1.jpg",       "nr": 1     },     {       "id": 308180,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-2.jpg",       "nr": 2     },     {       "id": 308181,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-3.jpg",       "nr": 3     }     ...   ] } ``` |
| --- | --- |

## [Search for Videos](https://www.pexels.com/api/documentation#videos-search)

### GET https://api.pexels.com/v1/videos/search

This endpoint enables you to search Pexels for any topic that you would like. For example your query could be something broad like `Nature`, `Tigers`, `People`. Or it could be something specific like `Group of people working`.

#### [Parameters](https://www.pexels.com/api/documentation#videos-search__parameters)

#### [Response](https://www.pexels.com/api/documentation#videos-search__response)

```bash
curl -H "Authorization: YOUR_API_KEY" \
  "https://api.pexels.com/v1/videos/search?query=nature&per_page=1"
```

Example Response

| ``` 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 114 115 116 117 118 119 120 121 122 123 124 125 126 127 128 129 130 131 132 133 134 135 136 137 138 139 140 141 142 143 144 145 146 147 148 149 150 151 152 153 154 155 156 ``` | ``` {   "page": 1,   "per_page": 1,   "total_results": 20475,   "url": "https://www.pexels.com/videos/",   "videos": [     {       "id": 1448735,       "width": 4096,       "height": 2160,       "url": "https://www.pexels.com/video/video-of-forest-1448735/",       "image": "https://images.pexels.com/videos/1448735/free-video-1448735.jpg?fit=crop&w=1200&h=630&auto=compress&cs=tinysrgb",       "duration": 32,       "user": {         "id": 574687,         "name": "Ruvim Miksanskiy",         "url": "https://www.pexels.com/@digitech"       },       "video_files": [         {           "id": 58649,           "quality": "sd",           "file_type": "video/mp4",           "width": 640,           "height": 338,           "link": "https://player.vimeo.com/external/291648067.sd.mp4?s=7f9ee1f8ec1e5376027e4a6d1d05d5738b2fbb29&profile_id=164&oauth2_token_id=57447761"         },         {           "id": 58650,           "quality": "hd",           "file_type": "video/mp4",           "width": 2048,           "height": 1080,           "link": "https://player.vimeo.com/external/291648067.hd.mp4?s=94998971682c6a3267e4cbd19d16a7b6c720f345&profile_id=175&oauth2_token_id=57447761"         },         {           "id": 58651,           "quality": "hd",           "file_type": "video/mp4",           "width": 4096,           "height": 2160,           "link": "https://player.vimeo.com/external/291648067.hd.mp4?s=94998971682c6a3267e4cbd19d16a7b6c720f345&profile_id=172&oauth2_token_id=57447761"         },         {           "id": 58652,           "quality": "hd",           "file_type": "video/mp4",           "width": 1366,           "height": 720,           "link": "https://player.vimeo.com/external/291648067.hd.mp4?s=94998971682c6a3267e4cbd19d16a7b6c720f345&profile_id=174&oauth2_token_id=57447761"         },         {           "id": 58653,           "quality": "hd",           "file_type": "video/mp4",           "width": 2732,           "height": 1440,           "link": "https://player.vimeo.com/external/291648067.hd.mp4?s=94998971682c6a3267e4cbd19d16a7b6c720f345&profile_id=170&oauth2_token_id=57447761"         },         {           "id": 58654,           "quality": "sd",           "file_type": "video/mp4",           "width": 960,           "height": 506,           "link": "https://player.vimeo.com/external/291648067.sd.mp4?s=7f9ee1f8ec1e5376027e4a6d1d05d5738b2fbb29&profile_id=165&oauth2_token_id=57447761"         },         {           "id": 58655,           "quality": "hls",           "file_type": "video/mp4",           "width": null,           "height": null,           "link": "https://player.vimeo.com/external/291648067.m3u8?s=1210fac9d80f9b74b4a334c4fca327cde08886b2&oauth2_token_id=57447761"         }       ],       "video_pictures": [         {           "id": 133236,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-0.jpg",           "nr": 0         },         {           "id": 133237,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-1.jpg",           "nr": 1         },         {           "id": 133238,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-2.jpg",           "nr": 2         },         {           "id": 133239,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-3.jpg",           "nr": 3         },         {           "id": 133240,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-4.jpg",           "nr": 4         },         {           "id": 133241,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-5.jpg",           "nr": 5         },         {           "id": 133242,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-6.jpg",           "nr": 6         },         {           "id": 133243,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-7.jpg",           "nr": 7         },         {           "id": 133244,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-8.jpg",           "nr": 8         },         {           "id": 133245,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-9.jpg",           "nr": 9         },         {           "id": 133246,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-10.jpg",           "nr": 10         },         {           "id": 133247,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-11.jpg",           "nr": 11         },         {           "id": 133248,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-12.jpg",           "nr": 12         },         {           "id": 133249,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-13.jpg",           "nr": 13         },         {           "id": 133250,           "picture": "https://static-videos.pexels.com/videos/1448735/pictures/preview-14.jpg",           "nr": 14         }       ]     }   ] } ``` |
| --- | --- |

```bash
curl -H "Authorization: YOUR_API_KEY" \
  "https://api.pexels.com/v1/videos/popular?per_page=1"
```

Example Response

| ``` 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 114 115 116 117 118 119 120 121 122 123 124 125 126 127 128 129 130 131 132 133 134 135 136 137 138 139 140 ``` | ``` {   "page": 1,   "per_page": 1,   "total_results": 4089,   "url": "https://www.pexels.com/search/videos/Nature/",   "videos": [     {       "id": 1093662,       "width": 1920,       "height": 1080,       "url": "https://www.pexels.com/video/water-crashing-over-the-rocks-1093662/",       "image": "https://images.pexels.com/videos/1093662/free-video-1093662.jpg?fit=crop&w=1200&h=630&auto=compress&cs=tinysrgb",       "duration": 8,       "user": {         "id": 417939,         "name": "Peter Fowler",         "url": "https://www.pexels.com/@peter-fowler-417939"       },       "video_files": [         {           "id": 37101,           "quality": "hd",           "file_type": "video/mp4",           "width": 1280,           "height": 720,           "link": "https://player.vimeo.com/external/269971860.hd.mp4?s=eae965838585cc8342bb5d5253d06a52b2415570&profile_id=174&oauth2_token_id=57447761"         },         {           "id": 37102,           "quality": "sd",           "file_type": "video/mp4",           "width": 640,           "height": 360,           "link": "https://player.vimeo.com/external/269971860.sd.mp4?s=a3036bd1a9f15c1b31daedad98c06a3b24cdd747&profile_id=164&oauth2_token_id=57447761"         },         {           "id": 37103,           "quality": "hd",           "file_type": "video/mp4",           "width": 1920,           "height": 1080,           "link": "https://player.vimeo.com/external/269971860.hd.mp4?s=eae965838585cc8342bb5d5253d06a52b2415570&profile_id=175&oauth2_token_id=57447761"         },         {           "id": 37104,           "quality": "sd",           "file_type": "video/mp4",           "width": 960,           "height": 540,           "link": "https://player.vimeo.com/external/269971860.sd.mp4?s=a3036bd1a9f15c1b31daedad98c06a3b24cdd747&profile_id=165&oauth2_token_id=57447761"         },         {           "id": 37105,           "quality": "hls",           "file_type": "video/mp4",           "width": null,           "height": null,           "link": "https://player.vimeo.com/external/269971860.m3u8?s=ac08929c597387cc77ae3d88bfe2ad66a9c4d31f&oauth2_token_id=57447761"         }       ],       "video_pictures": [         {           "id": 79696,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-0.jpg",           "nr": 0         },         {           "id": 79697,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-1.jpg",           "nr": 1         },         {           "id": 79698,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-2.jpg",           "nr": 2         },         {           "id": 79699,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-3.jpg",           "nr": 3         },         {           "id": 79700,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-4.jpg",           "nr": 4         },         {           "id": 79701,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-5.jpg",           "nr": 5         },         {           "id": 79702,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-6.jpg",           "nr": 6         },         {           "id": 79703,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-7.jpg",           "nr": 7         },         {           "id": 79704,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-8.jpg",           "nr": 8         },         {           "id": 79705,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-9.jpg",           "nr": 9         },         {           "id": 79706,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-10.jpg",           "nr": 10         },         {           "id": 79707,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-11.jpg",           "nr": 11         },         {           "id": 79708,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-12.jpg",           "nr": 12         },         {           "id": 79709,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-13.jpg",           "nr": 13         },         {           "id": 79710,           "picture": "https://static-videos.pexels.com/videos/1093662/pictures/preview-14.jpg",           "nr": 14         }       ]     }   ] } ``` |
| --- | --- |

## [Get a Video](https://www.pexels.com/api/documentation#videos-show)

### GET https://api.pexels.com/v1/videos/videos/:id

Retrieve a specific `Video` from its id.

#### [Parameters](https://www.pexels.com/api/documentation#videos-show__parameters)

#### [Response](https://www.pexels.com/api/documentation#videos-show__response)

Returns a `Video` object

```bash
curl -H "Authorization: YOUR_API_KEY" \
  "https://api.pexels.com/v1/videos/videos/2499611"
```

Example Response

| ``` 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 114 115 116 117 118 119 120 121 122 123 124 125 126 127 128 129 130 131 132 133 134 135 136 137 138 139 140 ``` | ``` {   "id": 2499611,   "width": 1080,   "height": 1920,   "url": "https://www.pexels.com/video/2499611/",   "image": "https://images.pexels.com/videos/2499611/free-video-2499611.jpg?fit=crop&w=1200&h=630&auto=compress&cs=tinysrgb",   "duration": 22,   "user": {     "id": 680589,     "name": "Joey Farina",     "url": "https://www.pexels.com/@joey"   },   "video_files": [     {       "id": 125004,       "quality": "hd",       "file_type": "video/mp4",       "width": 1080,       "height": 1920,       "link": "https://player.vimeo.com/external/342571552.hd.mp4?s=6aa6f164de3812abadff3dde86d19f7a074a8a66&profile_id=175&oauth2_token_id=57447761"     },     {       "id": 125005,       "quality": "sd",       "file_type": "video/mp4",       "width": 540,       "height": 960,       "link": "https://player.vimeo.com/external/342571552.sd.mp4?s=e0df43853c25598dfd0ec4d3f413bce1e002deef&profile_id=165&oauth2_token_id=57447761"     },     {       "id": 125006,       "quality": "sd",       "file_type": "video/mp4",       "width": 240,       "height": 426,       "link": "https://player.vimeo.com/external/342571552.sd.mp4?s=e0df43853c25598dfd0ec4d3f413bce1e002deef&profile_id=139&oauth2_token_id=57447761"     },     {       "id": 125007,       "quality": "hd",       "file_type": "video/mp4",       "width": 720,       "height": 1280,       "link": "https://player.vimeo.com/external/342571552.hd.mp4?s=6aa6f164de3812abadff3dde86d19f7a074a8a66&profile_id=174&oauth2_token_id=57447761"     },     {       "id": 125008,       "quality": "sd",       "file_type": "video/mp4",       "width": 360,       "height": 640,       "link": "https://player.vimeo.com/external/342571552.sd.mp4?s=e0df43853c25598dfd0ec4d3f413bce1e002deef&profile_id=164&oauth2_token_id=57447761"     },     {       "id": 125009,       "quality": "hls",       "file_type": "video/mp4",       "width": null,       "height": null,       "link": "https://player.vimeo.com/external/342571552.m3u8?s=53433233e4176eead03ddd6fea04d9fb2bce6637&oauth2_token_id=57447761"     }   ],   "video_pictures": [     {       "id": 308178,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-0.jpg",       "nr": 0     },     {       "id": 308179,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-1.jpg",       "nr": 1     },     {       "id": 308180,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-2.jpg",       "nr": 2     },     {       "id": 308181,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-3.jpg",       "nr": 3     },     {       "id": 308182,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-4.jpg",       "nr": 4     },     {       "id": 308183,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-5.jpg",       "nr": 5     },     {       "id": 308184,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-6.jpg",       "nr": 6     },     {       "id": 308185,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-7.jpg",       "nr": 7     },     {       "id": 308186,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-8.jpg",       "nr": 8     },     {       "id": 308187,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-9.jpg",       "nr": 9     },     {       "id": 308188,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-10.jpg",       "nr": 10     },     {       "id": 308189,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-11.jpg",       "nr": 11     },     {       "id": 308190,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-12.jpg",       "nr": 12     },     {       "id": 308191,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-13.jpg",       "nr": 13     },     {       "id": 308192,       "picture": "https://static-videos.pexels.com/videos/2499611/pictures/preview-14.jpg",       "nr": 14     }   ] } ``` |
| --- | --- |

## [The Collection Resource](https://www.pexels.com/api/documentation#collections-resource)

The `Collection` resource is a JSON formatted version of a Pexels collection. The Collection list endpoint responds with the collection data formatted in this shape.

#### [Response](https://www.pexels.com/api/documentation#collections-resource__response)

The Collection Resource

| ``` 1 2 3 4 5 6 7 8 9 ``` | ``` {   "id": "8xntbhr",   "title": "Hello Spring",   "description": "Baby chicks, rabbits & pretty flowers. What's not to love?",   "private": false,   "media_count": 130,   "photos_count": 121,   "videos_count": 9 } ``` |
| --- | --- |

## [Featured Collections](https://www.pexels.com/api/documentation#collections-featured)

### GET https://api.pexels.com/v1/collections/featured

This endpoint returns all featured collections on Pexels.

#### [Parameters](https://www.pexels.com/api/documentation#collections-featured__parameters)

#### [Response](https://www.pexels.com/api/documentation#collections-featured__response)

```bash
curl -H "Authorization: YOUR_API_KEY" \
  "https://api.pexels.com/v1/collections/featured?per_page=1"
```

Example Response

| ``` 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 ``` | ``` {   "collections": [     {       "id": "9mp14cx",       "title": "Cool Cats",       "description": null,       "private": false,       "media_count": 6,       "photos_count": 5,       "videos_count": 1     }   ],    "page": 2,   "per_page": 1,   "total_results": 5,   "next_page": "https://api.pexels.com/v1/collections/featured/?page=3&per_page=1",   "prev_page": "https://api.pexels.com/v1/collections/featured?page=1&per_page=1" } ``` |
| --- | --- |

## [My Collections](https://www.pexels.com/api/documentation#collections-all)

### GET https://api.pexels.com/v1/collections

This endpoint returns all of your collections.

#### [Parameters](https://www.pexels.com/api/documentation#collections-all__parameters)

#### [Response](https://www.pexels.com/api/documentation#collections-all__response)

```bash
curl -H "Authorization: YOUR_API_KEY" \
  "https://api.pexels.com/v1/collections?per_page=1"
```

Example Response

| ``` 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 ``` | ``` {   "collections": [     {       "id": "9mp14cx",       "title": "Cool Cats",       "description": null,       "private": false,       "media_count": 6,       "photos_count": 5,       "videos_count": 1     }   ],    "page": 2,   "per_page": 1,   "total_results": 5,   "next_page": "https://api.pexels.com/v1/collections/?page=3&per_page=1",   "prev_page": "https://api.pexels.com/v1/collections/?page=1&per_page=1" } ``` |
| --- | --- |

## [Collection Media](https://www.pexels.com/api/documentation#collections-media)

### GET https://api.pexels.com/v1/collections/:id

This endpoint returns all the media (photos and videos) within a single collection. You can filter to only receive photos or videos using the `type` parameter.

#### [Parameters](https://www.pexels.com/api/documentation#collections-media__parameters)

#### [Response](https://www.pexels.com/api/documentation#collections-media__response)

```bash
curl -H "Authorization: YOUR_API_KEY" \
  "https://api.pexels.com/v1/collections/9mp14cx?per_page=1&sort=desc"
```

Example Response

| ``` 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 114 115 116 117 118 119 120 121 122 123 124 125 126 127 128 129 130 131 132 133 134 135 136 137 138 139 140 141 142 143 144 145 146 147 148 149 150 151 152 153 154 155 156 157 158 159 160 ``` | ``` {   "id": "9mp14cx",   "media": [     {       "type": "Photo",       "id": 2061057,       "width": 4850,       "height": 3224,       "url": "https://www.pexels.com/photo/gray-and-white-kitten-on-white-bed-2061057/",       "photographer": "Tranmautritam",       "photographer_url": "https://www.pexels.com/@tranmautritam",       "photographer_id": 8509,       "avg_color": "#BBBEC3",       "src": {         "original": "https://images.pexels.com/photos/2061057/pexels-photo-2061057.jpeg",         "large2x": "https://images.pexels.com/photos/2061057/pexels-photo-2061057.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940",         "large": "https://images.pexels.com/photos/2061057/pexels-photo-2061057.jpeg?auto=compress&cs=tinysrgb&h=650&w=940",         "medium": "https://images.pexels.com/photos/2061057/pexels-photo-2061057.jpeg?auto=compress&cs=tinysrgb&h=350",         "small": "https://images.pexels.com/photos/2061057/pexels-photo-2061057.jpeg?auto=compress&cs=tinysrgb&h=130",         "portrait": "https://images.pexels.com/photos/2061057/pexels-photo-2061057.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800",         "landscape": "https://images.pexels.com/photos/2061057/pexels-photo-2061057.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200",         "tiny": "https://images.pexels.com/photos/2061057/pexels-photo-2061057.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280"       },       "liked": false     },     {       "type": "Video",       "id": 854982,       "width": 1280,       "height": 720,       "duration": 11,       "full_res": null,       "tags": [],       "url": "https://www.pexels.com/video/video-of-a-tabby-cat-854982/",       "image": "https://images.pexels.com/videos/854982/free-video-854982.jpg?auto=compress&cs=tinysrgb&fit=crop&h=630&w=1200",       "avg_color": null,       "user": {         "id": 2659,         "name": "Pixabay",         "url": "https://www.pexels.com/@pixabay"       },       "video_files": [         {           "id": 17755,           "quality": "hd",           "file_type": "video/mp4",           "width": 1280,           "height": 720,           "link": "https://player.vimeo.com/external/199433617.hd.mp4?s=1770018c20604d41d60e4f574e7680a1bd15edb8&profile_id=174&oauth2_token_id=57447761"         },         {           "id": 17756,           "quality": "sd",           "file_type": "video/mp4",           "width": 640,           "height": 360,           "link": "https://player.vimeo.com/external/199433617.sd.mp4?s=457abd2452a52548b8c02c503a91035ce8a713f0&profile_id=164&oauth2_token_id=57447761"         },         {           "id": 17757,           "quality": "sd",           "file_type": "video/mp4",           "width": 960,           "height": 540,           "link": "https://player.vimeo.com/external/199433617.sd.mp4?s=457abd2452a52548b8c02c503a91035ce8a713f0&profile_id=165&oauth2_token_id=57447761"         },         {           "id": 17758,           "quality": "hls",           "file_type": "video/mp4",           "width": null,           "height": null,           "link": "https://player.vimeo.com/external/199433617.m3u8?s=115ec8875069ea6203ddca51dae78cebd934b86e&oauth2_token_id=57447761"         }       ],       "video_pictures": [         {           "id": 19591,           "nr": 0,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-0.jpg"         },         {           "id": 19592,           "nr": 1,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-1.jpg"         },         {           "id": 19593,           "nr": 2,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-2.jpg"         },         {           "id": 19594,           "nr": 3,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-3.jpg"         },         {           "id": 19595,           "nr": 4,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-4.jpg"         },         {           "id": 19596,           "nr": 5,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-5.jpg"         },         {           "id": 19597,           "nr": 6,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-6.jpg"         },         {           "id": 19598,           "nr": 7,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-7.jpg"         },         {           "id": 19599,           "nr": 8,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-8.jpg"         },         {           "id": 19600,           "nr": 9,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-9.jpg"         },         {           "id": 19601,           "nr": 10,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-10.jpg"         },         {           "id": 19602,           "nr": 11,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-11.jpg"         },         {           "id": 19603,           "nr": 12,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-12.jpg"         },         {           "id": 19604,           "nr": 13,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-13.jpg"         },         {           "id": 19605,           "nr": 14,           "picture": "https://images.pexels.com/videos/854982/pictures/preview-14.jpg"         }       ]     }   ],   "page": 2,   "per_page": 2,   "total_results": 6,   "next_page": "https://api.pexels.com/v1/collections/9mp14cx/?page=3&per_page=2",   "prev_page": "https://api.pexels.com/v1/collections/9mp14cx/?page=1&per_page=2" } ``` |
| --- | --- |

## [Changelog](https://www.pexels.com/api/documentation#changelog)

This is a list of major changes to the Pexels API.

#### 2023-11-22

- Added `sort` query parameter to the `/collections/:id` endpoint.

#### 2022-09-15

- Added `video_file.fps` attribute for the `Video` resource.

#### 2021-12-14

- Added `alt` attribute to the `Photo` resource.
- Added previously-exposed `liked` attribute to the `Photo` responses.

#### 2021-09-13

- Updated `image` attribute for the `Video` resource to use the correct orientation.
- Updated `video_picture.picture` attribute for the `Video` resource to use the correct orientation.

#### 2021-08-12

- Added Featured Collections endpoint.
- Updated `/collections/:id`. Returns a collection if the collection is `featured` or belongs to the authenticated user.

#### 2021-04-19

- Added Collections resource and endpoints.

#### 2020-12-11

- Added `avg_color` attribute to the Photo resource.

#### 2020-11-12

- Added `orientation`, `size` and `color` filters to Photo Search.
- Added `orientation` and `size` filters to Video Search.

#### 2020-05-28

- Initial version of this documentation.
