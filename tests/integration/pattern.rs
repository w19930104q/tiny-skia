use tiny_skia::*;

fn crate_triangle() -> Pixmap {
    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut pb = PathBuilder::new();
    pb.move_to(0.0, 20.0);
    pb.line_to(20.0, 20.0);
    pb.line_to(10.0, 0.0);
    pb.close();
    let path = pb.finish().unwrap();

    let mut pixmap = Pixmap::new(20, 20).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    pixmap
}

#[test]
fn pad_nearest() {
    let triangle = crate_triangle();

    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Pad,
        FilterQuality::Nearest,
        1.0,
        Transform::identity(),
    );

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/pattern/pad-nearest.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn repeat_nearest() {
    let triangle = crate_triangle();

    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Repeat,
        FilterQuality::Nearest,
        1.0,
        Transform::identity(),
    );

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/pattern/repeat-nearest.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn reflect_nearest() {
    let triangle = crate_triangle();

    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Reflect,
        FilterQuality::Nearest,
        1.0,
        Transform::identity(),
    );

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/pattern/reflect-nearest.png").unwrap();
    assert_eq!(pixmap, expected);
}

// We have to test tile mode for bilinear/bicubic separately,
// because they're using a different algorithm from nearest.
#[test]
fn pad_bicubic() {
    let triangle = crate_triangle();

    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Pad,
        FilterQuality::Bicubic,
        1.0,
        // Transform must be set, otherwise we will fallback to Nearest.
        Transform::from_row(1.1, 0.3, 0.0, 1.4, 0.0, 0.0),
    );

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/pattern/pad-bicubic.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn repeat_bicubic() {
    let triangle = crate_triangle();

    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Repeat,
        FilterQuality::Bicubic,
        1.0,
        // Transform must be set, otherwise we will fallback to Nearest.
        Transform::from_row(1.1, 0.3, 0.0, 1.4, 0.0, 0.0),
    );

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/pattern/repeat-bicubic.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn reflect_bicubic() {
    let triangle = crate_triangle();

    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Reflect,
        FilterQuality::Bicubic,
        1.0,
        // Transform must be set, otherwise we will fallback to Nearest.
        Transform::from_row(1.1, 0.3, 0.0, 1.4, 0.0, 0.0),
    );

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/pattern/reflect-bicubic.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn filter_nearest_no_ts() {
    let triangle = crate_triangle();

    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Repeat,
        FilterQuality::Nearest,
        1.0,
        Transform::identity(),
    );

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/pattern/filter-nearest-no-ts.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn filter_nearest() {
    let triangle = crate_triangle();

    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Repeat,
        FilterQuality::Nearest,
        1.0,
        Transform::from_row(1.5, 0.0, -0.4, -0.8, 5.0, 1.0),
    );

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/pattern/filter-nearest.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn filter_bilinear() {
    let triangle = crate_triangle();

    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Repeat,
        FilterQuality::Bilinear,
        1.0,
        Transform::from_row(1.5, 0.0, -0.4, -0.8, 5.0, 1.0),
    );

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/pattern/filter-bilinear.png").unwrap();
    assert_eq!(pixmap, expected);
}

#[test]
fn filter_bicubic() {
    let triangle = crate_triangle();

    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Repeat,
        FilterQuality::Bicubic,
        1.0,
        Transform::from_row(1.5, 0.0, -0.4, -0.8, 5.0, 1.0),
    );

    let path = PathBuilder::from_rect(Rect::from_ltrb(10.0, 10.0, 190.0, 190.0).unwrap());

    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

    let expected = Pixmap::load_png("tests/images/pattern/filter-bicubic.png").unwrap();
    assert_eq!(pixmap, expected);
}

// Extreme/singular transform handling.
//
// The raster pipeline derives sampler indices and interpolation weights from
// f32 coordinates with saturating float-to-int conversions. Singular matrices
// must be rejected and extreme scales must fall back to the nearest-neighbor
// sampler instead of producing NaN weights (which rendered as fully
// transparent) or garbled pixels. See `shaders::invert_shader_transform` and
// `shaders::is_filterable`.

fn quad_pixmap() -> Pixmap {
    let mut pm = Pixmap::new(8, 8).unwrap();
    let colors = [(200u8, 50u8, 50u8), (50, 200, 50), (50, 50, 200), (200, 200, 50)];
    for (i, (r, g, b)) in colors.iter().enumerate() {
        let mut paint = Paint::default();
        paint.set_color_rgba8(*r, *g, *b, 255);
        let x = (i % 2) as f32 * 4.0;
        let y = (i / 2) as f32 * 4.0;
        pm.fill_rect(
            Rect::from_ltrb(x, y, x + 4.0, y + 4.0).unwrap(),
            &paint,
            Transform::identity(),
            None,
        );
    }
    pm
}

fn render_pattern(ts: Transform, quality: FilterQuality, force_hq: bool) -> Pixmap {
    let src = quad_pixmap();
    let mut paint = Paint::default();
    paint.anti_alias = false;
    paint.force_hq_pipeline = force_hq;
    paint.shader = Pattern::new(src.as_ref(), SpreadMode::Pad, quality, 1.0, ts);

    let path = PathBuilder::from_rect(Rect::from_ltrb(0.0, 0.0, 32.0, 32.0).unwrap());
    let mut pixmap = Pixmap::new(32, 32).unwrap();
    pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
    pixmap
}

#[test]
fn extreme_downscale_does_not_render_transparent() {
    // Used to produce all-zero output because `fract` overflowed and the
    // bilinear/bicubic weights became NaN.
    for quality in [FilterQuality::Bilinear, FilterQuality::Bicubic] {
        let pixmap = render_pattern(Transform::from_scale(1e-30, 1e-30), quality, false);
        assert!(pixmap.data().iter().any(|&b| b != 0), "{quality:?} produced empty output");
    }
}

#[test]
fn extreme_upscale_does_not_render_transparent() {
    for quality in [
        FilterQuality::Nearest,
        FilterQuality::Bilinear,
        FilterQuality::Bicubic,
    ] {
        let pixmap = render_pattern(Transform::from_scale(1e20, 1e20), quality, false);
        assert!(pixmap.data().iter().any(|&b| b != 0), "{quality:?} produced empty output");
    }
}

#[test]
fn singular_pattern_transform_renders_nothing() {
    let cases = [
        Transform::from_scale(0.0, 1.0),
        Transform::from_scale(1.0, 0.0),
        Transform::from_row(1.0, 0.0, 1.0, 0.0, 0.0, 0.0),
        Transform::from_scale(f32::INFINITY, 1.0),
        Transform::from_scale(f32::NAN, 1.0),
    ];

    for ts in cases {
        for quality in [
            FilterQuality::Nearest,
            FilterQuality::Bilinear,
            FilterQuality::Bicubic,
        ] {
            let pixmap = render_pattern(ts, quality, false);
            assert!(
                pixmap.data().iter().all(|&b| b == 0),
                "{ts:?} {quality:?} must not draw anything"
            );
        }
    }
}

#[test]
fn extreme_scales_force_hq_matches_auto() {
    for scale in [1e-30f32, 1e-20, 1e-6, 1e6, 1e20] {
        let ts = Transform::from_scale(scale, scale);
        for quality in [
            FilterQuality::Nearest,
            FilterQuality::Bilinear,
            FilterQuality::Bicubic,
        ] {
            assert_eq!(
                render_pattern(ts, quality, false).data(),
                render_pattern(ts, quality, true).data(),
                "scale={scale} quality={quality:?}"
            );
        }
    }
}

#[test]
fn moderate_scale_keeps_interpolation() {
    // The fallback must only kick in past the safe coordinate range; a regular
    // upscale must still go through bilinear interpolation.
    let nearest = render_pattern(Transform::from_scale(2.5, 2.5), FilterQuality::Nearest, false);
    let bilinear = render_pattern(Transform::from_scale(2.5, 2.5), FilterQuality::Bilinear, false);
    assert_ne!(nearest.data(), bilinear.data());
}
