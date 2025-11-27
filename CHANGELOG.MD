# egui Double Slider Widget changelog

All notable changes to the `egui double slider` crate will be documented in this file.

# Unreleased 1.X.Y - X.X.2025

### Added:

* ...

# 1.0.0 - 28.11.2025

* Update to egui 0.33

# 0.9.0 - 12.7.2025

### Added:

* Update to egui 0.32

# 0.8.0 - 6.7.2025

### Added:

* Changed default styles to match active egui visuals
* Added `egui-theme-switch` to the example

# 0.7.3 - 19.6.2025

### Added:

* Fixed scroll and zoom factors

# 0.7.2 - 10.6.2025

### Added:

* Ability to select horizontal or vertical scroll axis.

# 0.7.1 - 23.5.2025

### Added:

* Fix for the case where range is specified as `0.0..=0.0` or any other single valued range.

# 0.7.0 - 24.3.2025

### Added:

* Pass through responses of all components (points, in-between section) to allow catching double clicks etc...

# 0.6.0 - 24.3.2025

### Added:

* Handle any numeric type (thanks to [@g-pascal](https://github.com/g-pascal))
* Add option to use logarithmic scale (thanks to [@g-pascal](https://github.com/g-pascal))
* Add option to prevent dragging a value beyond the other one (thanks to [@g-pascal](https://github.com/g-pascal))

### Fixed:

* Make the "in_between" drag zone smaller to avoid overlapping with the handles (thanks
  to [@g-pascal](https://github.com/g-pascal))
* Prevent scroll and zoom from setting a value outside the allowed range (thanks
  to [@g-pascal](https://github.com/g-pascal))

### Changed:

* Set default separation_distance to 1.0

# 0.5.0 - 20.2.2025

### Added:

* move `eframe` to dev-dependencies

# 0.4.0 - 9.2.2025

### Added:

* Add example instructions
* Update to egui 0.31

## 0.3.2 - 5.12.2024

### Added:

* Added github actions for auto-publishing on crates.io

## 0.3.0 - 1.11.2024 - Scrolling and zooming implemented

### Added:

* Scrolling and zooming gesture with configurable factors in the builder


