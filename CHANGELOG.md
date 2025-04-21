## [Unreleased]

## [0.0.2] - 2025-04-21

- Adds Rails support via a ActionView template handler.
  Takes views like `action.show.erb`, pre-processes them with Erb and renders
  to a .png with 'image/png' set as the MIME type.
- Adds a `ActionController::Renderer` so `render himg: '<div>Some HTML</div>'`
  can be used either directly or from a `format.png`/`format.himg` block.

## [0.0.1] - 2025-04-19

- Initial alpha
