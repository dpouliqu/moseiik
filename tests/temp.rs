use moseiik::main::{compute_mosaic, Options};

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: &str = "assets/kit.jpeg";
    const TILES: &str = "assets/images";
    const GROUND_TRUTH: &str = "assets/ground-truth-kit.png";

    // Tiles are loaded in parallel, so ties in find_best_tile may be resolved
    // differently across runs (~0.13% of pixels measured). A strict == would be
    // flaky, so we allow a small ratio of differing pixels (well below what a
    // real bug would produce).
    const MAX_DIFF_RATIO: f64 = 0.01; // 1 %

    // Runs the full mosaic pipeline and compares the output to the ground truth.
    // `out_name` is unique per test to avoid file clashes when tests run in parallel.
    fn assert_mosaic_matches_ground_truth(simd: bool, out_name: &str) {
        let mut out_path = std::env::temp_dir();
        out_path.push(out_name);
        let output = out_path.to_str().expect("chemin de sortie invalide").to_string();

        let args = Options {
            image: TARGET.to_string(),
            output: output.clone(),
            tiles: TILES.to_string(),
            scaling: 1,
            tile_size: 25, // ground truth was generated with tile_size 25, scaling 1
            remove_used: false,
            verbose: false,
            simd,
            num_thread: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
        };

        // compute_mosaic returns nothing: it writes the result to `output`.
        compute_mosaic(args);

        let generated = image::open(&output)
            .expect("La mosaique generee doit etre lisible")
            .into_rgb8();
        let ground_truth = image::open(GROUND_TRUTH)
            .expect("La verite terrain doit etre lisible")
            .into_rgb8();

        // Dimensions are deterministic, so they must match exactly.
        assert_eq!(
            generated.dimensions(),
            ground_truth.dimensions(),
            "Les dimensions de la mosaique different de la verite terrain"
        );

        // Content: count differing pixels and check the ratio stays under the threshold.
        let total = (generated.width() * generated.height()) as f64;
        let differing = generated
            .pixels()
            .zip(ground_truth.pixels())
            .filter(|(a, b)| a != b)
            .count() as f64;
        let ratio = differing / total;

        assert!(
            ratio <= MAX_DIFF_RATIO,
            "Trop de pixels differents : {:.3} % (> seuil {:.3} %)",
            ratio * 100.0,
            MAX_DIFF_RATIO * 100.0
        );
    }

    /// Integration test on x86: forces the SIMD path (SSE2).
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        assert_mosaic_matches_ground_truth(true, "moseiik_out_x86.png");
    }

    /// Integration test on ARM64: forces the SIMD path (NEON).
    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        assert_mosaic_matches_ground_truth(true, "moseiik_out_aarch64.png");
    }

    /// Integration test without SIMD: runs on every architecture.
    #[test]
    fn test_generic() {
        assert_mosaic_matches_ground_truth(false, "moseiik_out_generic.png");
    }
}
