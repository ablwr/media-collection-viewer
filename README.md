# Media Collection Viewer

Early WIP!

Demo is [live](http://bits.ashleyblewer.com/media-collection-viewer/index.html)

## Description

Upload a mediainfo.json (`mediainfo ~/path/to/audiovideo/files/* --Output=JSON > mediainfo.json`) to get charts that provide you with a high-level overview of your collections.

**WIP -- Only one chart so far! 😅**

## Project goals

- Fast (and not fall over or freeze trying to process data)
- Sturdy (not needing a lot of assistance)
- Client-side only / no sending data anywhere (safe to use without contacting your org's lawyers)
- Deployable for free (no breaking my bank)
- Easy to use (anyone with the right data should be able to use)

## Charts

- [x] How many tracks are in each file?
- [x] What kind of formats?
- [x] What kind of audio codecs?
- [x] What kind of video codecs?
- [] What are the duration outliers (videos very short or very long)?
- [] What are the dimensions?
- [] What are the color spaces?
- [] What are the bit depths (color)?
- [] What are the bit dephts (audio)?
- [] Which files have timecodes?
- [] Standard? (NTSC, PAL, Other)
- [] Any files in which audio duration and video duration differ?

## TODO list

- [x] Test uploaded JS can roundtrip through WASM to Rust and back to browser  
- [x] Build JSON in Rust to send to browser  
- [x] Build chart out of delivered JSON
- [x] warn if not getting desired input
- [x] let user know when file is loading and loaded
- [] refactor!
- [] cleaner delivery of JS objects to charts
- [] cleaner display of charts (update, not overwrite)
- [] utilize 'about' page
- [] implement json fields for Tracks
- [] investigate other visualization options -- not totally sold on chart.js, a rust library would be cool!
- [] See src/routes/home.rs for additional features/charts to build out after proof of concept has been tested
- [] Move TODOs to Issues tracker

## Code
- Rust
- WASM
- [Yew](https://github.com/yewstack/yew) (with [starter app](https://github.com/jetli/create-yew-app))
- JavaScript
- [ChartJS](https://www.chartjs.org/)
- CSS 😘


# yew documentation

## How to install

```sh
npm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## How to run unit tests

```sh
# Runs tests
npm test
```