use crate::root_io::RootCapability;
use kamacite_core::{
    AdapterIdentity, AdmittedNickelJsonExport, CanonicalValueIdentity, IdentityDomain,
    NickelExportIdentity, NickelExportIdentityKind, NickelPreservesReceipt,
    PRESERVES_CANONICAL_CODEC, ProjectionIdentity, ReceiptIdentity, WireIdentity,
    adapt_admitted_nickel_json_export, compute_identity,
};
use nickel_export_core::{
    ArtifactMaterial, ExportFormat, ExportManifest, admit_manifest, verify_supplied_artifacts,
};
use serde::Serialize;
use std::path::{Path, PathBuf};

const RECEIPT_TRAILING_NEWLINE: u8 = b'\n';

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdaptMode {
    Check,
    Write,
}

#[derive(Clone, Debug)]
pub struct NickelPreservesCommand {
    pub root: PathBuf,
    pub manifest: PathBuf,
    pub selected_export: String,
    pub artifact: PathBuf,
    pub packed_output: PathBuf,
    pub text_output: Option<PathBuf>,
    pub receipt: PathBuf,
    pub profile: PathBuf,
    pub canonical_codec: String,
    pub mode: AdaptMode,
}

#[derive(Clone, Debug, Serialize)]
pub struct NickelPreservesCommandReport {
    pub passed: bool,
    pub mode: String,
    pub selected_export: String,
    pub manifest_wire_identity: WireIdentity,
    pub json_projection_identity: ProjectionIdentity,
    pub mapping_profile_identity: AdapterIdentity,
    pub packed_preserves_identity: CanonicalValueIdentity,
    pub review_text_identity: ProjectionIdentity,
    pub receipt_identity: ReceiptIdentity,
    pub packed_output: String,
    pub text_output: Option<String>,
    pub receipt_output: String,
    pub checked_laws: Vec<String>,
    pub non_claims: Vec<String>,
}

// r[impl kamacite.nickel_preserves_adapter.admission]
// r[impl kamacite.nickel_preserves_adapter.composition]
// r[impl kamacite.nickel_preserves_adapter.command]
pub fn run_nickel_preserves_command(
    command: &NickelPreservesCommand,
) -> Result<NickelPreservesCommandReport, String> {
    validate_distinct_paths(command)?;
    let root = RootCapability::open(&command.root).map_err(|error| error.to_string())?;
    let manifest_bytes = root
        .read(&command.manifest)
        .map_err(|error| error.to_string())?;
    let manifest_wire = serde_json::from_slice::<ExportManifest>(&manifest_bytes)
        .map_err(|error| format!("nickel-export manifest parse failed: {error}"))?;
    let manifest = admit_manifest(manifest_wire)
        .map_err(|error| format!("nickel-export manifest admission failed: {error}"))?;
    let selected = select_json_export(&manifest.exports, &command.selected_export)?;
    let json_bytes = root
        .read(&command.artifact)
        .map_err(|error| error.to_string())?;
    let material = [ArtifactMaterial {
        path: &selected.output.path,
        bytes: &json_bytes,
    }];
    verify_supplied_artifacts(&manifest, &material)
        .map_err(|error| format!("nickel-export artifact verification failed: {error}"))?;
    let manifest_wire_identity = compute_identity(IdentityDomain::Adapter, &manifest_bytes);
    let admitted = normalize_admitted_export(
        selected,
        &manifest,
        manifest_wire_identity.clone(),
        json_bytes,
    )?;
    let profile_bytes = root
        .read(&command.profile)
        .map_err(|error| error.to_string())?;
    let adaptation =
        adapt_admitted_nickel_json_export(&admitted, &profile_bytes, &command.canonical_codec)
            .map_err(|error| format!("Nickel Preserves adaptation failed: {error}"))?;
    let receipt_bytes = render_receipt(&adaptation.receipt)?;
    let text_bytes = adaptation.canonical_artifact.text_projection.as_bytes();
    match command.mode {
        AdaptMode::Check => check_outputs(
            &root,
            command,
            &adaptation.canonical_artifact.packed_bytes,
            text_bytes,
            &receipt_bytes,
        )?,
        AdaptMode::Write => write_outputs(
            &root,
            command,
            &adaptation.canonical_artifact.packed_bytes,
            text_bytes,
            &receipt_bytes,
        )?,
    }
    Ok(NickelPreservesCommandReport {
        passed: true,
        mode: match command.mode {
            AdaptMode::Check => "check",
            AdaptMode::Write => "write",
        }
        .to_string(),
        selected_export: command.selected_export.clone(),
        manifest_wire_identity,
        json_projection_identity: adaptation.receipt.body.json_projection_identity.clone(),
        mapping_profile_identity: adaptation.profile_identity,
        packed_preserves_identity: adaptation.packed_preserves_identity,
        review_text_identity: adaptation.review_text_identity,
        receipt_identity: adaptation.receipt.receipt_identity,
        packed_output: display_path(&command.packed_output),
        text_output: command.text_output.as_ref().map(|path| display_path(path)),
        receipt_output: display_path(&command.receipt),
        checked_laws: adaptation.receipt.body.checked_laws,
        non_claims: adaptation.receipt.body.non_claims,
    })
}

fn select_json_export<'a>(
    exports: &'a [nickel_export_core::ExportReceipt],
    selected_export: &str,
) -> Result<&'a nickel_export_core::ExportReceipt, String> {
    let mut matching = exports
        .iter()
        .filter(|receipt| receipt.output.path == selected_export);
    let selected = matching.next().ok_or_else(|| {
        "selected nickel-export destination is absent from the manifest".to_string()
    })?;
    if matching.next().is_some() {
        return Err("selected nickel-export destination is duplicated".to_string());
    }
    if selected.format != ExportFormat::Json {
        return Err("selected nickel-export destination is not format=json".to_string());
    }
    Ok(selected)
}

fn normalize_admitted_export(
    selected: &nickel_export_core::ExportReceipt,
    manifest: &nickel_export_core::VerifiedManifest,
    manifest_wire_identity: WireIdentity,
    json_bytes: Vec<u8>,
) -> Result<AdmittedNickelJsonExport, String> {
    Ok(AdmittedNickelJsonExport {
        manifest_schema: manifest.schema.clone(),
        receipt_schema: selected.schema.clone(),
        destination: selected.output.path.clone(),
        manifest_identity: external_identity(
            NickelExportIdentityKind::Manifest,
            &manifest.manifest_identity,
        )?,
        manifest_wire_identity,
        export_receipt_identity: external_identity(
            NickelExportIdentityKind::ExportReceipt,
            &selected.receipt_identity,
        )?,
        declared_input_identity: external_identity(
            NickelExportIdentityKind::DeclaredInput,
            &selected.declared_input_identity,
        )?,
        source_identity: external_identity(
            NickelExportIdentityKind::Source,
            &selected.source.identity,
        )?,
        evaluator_label: selected.evaluator.identity.clone(),
        evaluator_version: selected.evaluator.version.clone(),
        evaluator_artifact_identity: external_identity(
            NickelExportIdentityKind::EvaluatorArtifact,
            &selected.evaluator.artifact_identity,
        )?,
        evaluation_plan_identity: external_identity(
            NickelExportIdentityKind::EvaluationPlan,
            &selected.evaluator.plan_identity,
        )?,
        json_output_identity: external_identity(
            NickelExportIdentityKind::JsonOutput,
            &selected.output.identity,
        )?,
        json_bytes,
        upstream_non_claim: selected.non_claim.clone(),
    })
}

fn external_identity(
    kind: NickelExportIdentityKind,
    identity: &str,
) -> Result<NickelExportIdentity, String> {
    NickelExportIdentity::new(kind, identity.to_string()).map_err(|error| error.to_string())
}

fn render_receipt(receipt: &NickelPreservesReceipt) -> Result<Vec<u8>, String> {
    let mut bytes = serde_json::to_vec_pretty(receipt)
        .map_err(|error| format!("Nickel Preserves receipt render failed: {error}"))?;
    bytes.push(RECEIPT_TRAILING_NEWLINE);
    Ok(bytes)
}

fn check_outputs(
    root: &RootCapability,
    command: &NickelPreservesCommand,
    packed_bytes: &[u8],
    text_bytes: &[u8],
    receipt_bytes: &[u8],
) -> Result<(), String> {
    compare_output(
        root,
        &command.packed_output,
        packed_bytes,
        "packed Preserves",
    )?;
    if let Some(text_output) = &command.text_output {
        compare_output(root, text_output, text_bytes, "Preserves review text")?;
    }
    compare_output(root, &command.receipt, receipt_bytes, "adapter receipt")
}

fn write_outputs(
    root: &RootCapability,
    command: &NickelPreservesCommand,
    packed_bytes: &[u8],
    text_bytes: &[u8],
    receipt_bytes: &[u8],
) -> Result<(), String> {
    let packed_existing = root
        .read_optional(&command.packed_output)
        .map_err(|error| error.to_string())?;
    let text_existing = command
        .text_output
        .as_ref()
        .map(|path| root.read_optional(path).map_err(|error| error.to_string()))
        .transpose()?;
    let receipt_existing = root
        .read_optional(&command.receipt)
        .map_err(|error| error.to_string())?;
    let packed_matches = packed_existing.as_deref() == Some(packed_bytes);
    let text_matches = match (&command.text_output, &text_existing) {
        (None, None) => true,
        (Some(_), Some(existing)) => existing.as_deref() == Some(text_bytes),
        _ => false,
    };
    let receipt_matches = receipt_existing.as_deref() == Some(receipt_bytes);
    if packed_matches && text_matches && receipt_matches {
        return Err("Nickel Preserves write would be a no-op".to_string());
    }
    root.write(&command.packed_output, packed_bytes)
        .map_err(|error| error.to_string())?;
    if let Some(text_output) = &command.text_output {
        root.write(text_output, text_bytes)
            .map_err(|error| error.to_string())?;
    }
    // Publish the trusted receipt last. A prior failure cannot leave success evidence.
    root.write(&command.receipt, receipt_bytes)
        .map_err(|error| error.to_string())
}

fn compare_output(
    root: &RootCapability,
    path: &Path,
    expected: &[u8],
    label: &str,
) -> Result<(), String> {
    let actual = root
        .read_optional(path)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("{label} output is missing at {}", path.display()))?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{label} output is stale at {}", path.display()))
    }
}

fn validate_distinct_paths(command: &NickelPreservesCommand) -> Result<(), String> {
    let mut paths = vec![
        &command.manifest,
        &command.artifact,
        &command.packed_output,
        &command.receipt,
        &command.profile,
    ];
    if let Some(text_output) = &command.text_output {
        paths.push(text_output);
    }
    for (index, path) in paths.iter().enumerate() {
        if paths.iter().skip(index + 1).any(|other| other == path) {
            return Err(format!(
                "Nickel Preserves command paths must be distinct: {}",
                path.display()
            ));
        }
    }
    if command.canonical_codec != PRESERVES_CANONICAL_CODEC {
        return Err(
            "Nickel Preserves command requested an unsupported canonical codec".to_string(),
        );
    }
    Ok(())
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST_BYTES: &[u8] =
        include_bytes!("../../../fixtures/nickel-export/service-config/generated/manifest.json");
    const SELECTED_EXPORT: &str = "examples/service-config/generated/service.json";

    #[test]
    fn selects_exact_json_export_and_rejects_non_json_duplicate_and_missing() {
        let manifest = serde_json::from_slice::<ExportManifest>(MANIFEST_BYTES);
        assert!(manifest.is_ok());
        if let Ok(manifest) = manifest {
            assert!(select_json_export(&manifest.exports, SELECTED_EXPORT).is_ok());

            let first = manifest.exports.first().cloned();
            assert!(first.is_some());
            if let Some(first) = first {
                let mut non_json = manifest.exports.clone();
                if let Some(selected) = non_json.first_mut() {
                    selected.format = ExportFormat::Toml;
                }
                assert!(select_json_export(&non_json, SELECTED_EXPORT).is_err());

                let mut duplicate = manifest.exports.clone();
                duplicate.push(first);
                assert!(select_json_export(&duplicate, SELECTED_EXPORT).is_err());
            }

            assert!(select_json_export(&manifest.exports, "missing.json").is_err());
        }
    }
}
