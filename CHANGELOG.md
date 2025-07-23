## [Unreleased]

- Added `fetch_timeout` option to control restrict how long can be spent fetching resources.

## [0.0.6] - 2025-07-23

- Added bin/himg CLI screenshot tool
- Added `base_url` option for configuring relative paths
- Added `disable_fetch` option for security hardening
- Fixed race conditions when failing to fetch resources

## [0.0.5] - 2025-04-22

- Can configure `render himg: ""` with options including `width:`, `height:`,
  `truncate:` and `verbose:`.
- Can use `himg_config` helper methods at controller level and action level
  to set `@_himg_config`, which can then be passed to the renderer with
  `render himg: "<!DOCTYPE html>", config: himg_config` as an alternative to
  specifying config options individually to `ActionController::Rendering#render`.
- `himg_config` helpers can also be used to control the configuration of
  template based default render, when not calling render manually within the
  controller. This works because the template handler can access `@_himg_config`.
- Can use string width / height with Himg.render
- Disable detailed log timings by default. Can re-enable with verbose: true.
  There is still some logging from blitz-net#fetch for http requests and for
  blitz-html::DocumentHtmlParser#finish on unexpected tokens.

## [0.0.4] - 2025-04-22

- Allow `width`, `height` and `truncate` to be passed to the render function.

## [0.0.3] - 2025-04-22

- Ensure that when render height is expanded to fit the content that we update
  the height we try to write in the png metadata to match.
- Rust creates a ruby string with the binary png data so we don't need to convert with .pack("C*")
- Himg::Error wraps errors from a Rust Result<_,Err>
- Added OpenGraph metadata example
- Default dimensions of 720x405 to match the ideal 16:9 ratio and image size
  for sharing og-image on messengers and social media.

## [0.0.2] - 2025-04-21

- Adds Rails support via a ActionView template handler.
  Takes views like `action.show.erb`, pre-processes them with Erb and renders
  to a .png with 'image/png' set as the MIME type.
- Adds a `ActionController::Renderer` so `render himg: '<div>Some HTML</div>'`
  can be used either directly or from a `format.png`/`format.himg` block.

## [0.0.1] - 2025-04-19

- Initial alpha
