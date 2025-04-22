## [Unreleased]

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
