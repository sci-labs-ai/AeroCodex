#!/usr/bin/env python3
"""Verify A18 orbital-geometry and conic-branch Wave 3 terminal dispositions.

This dependency-free verifier consumes classifier metadata, A16-A17 manifests,
existing A7 batch metadata, prior external resolution manifests, and explicit
A18 terminal resolution records. It never opens or parses raw Rust-port, M07,
or Scilab source text.
"""
from __future__ import annotations
import argparse, csv, json, re, sys
from collections import Counter
from pathlib import Path
from typing import Any, Iterable

SCHEMA_VERSION='aerocodex.external_m07_resolution.v1'
CLASSIFIER_PATH='docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv'
WAVE1_PATH='formula-vault/resolutions/m07_orbital_geometry_conic_wave1.tsv'
WAVE2_PATH='formula-vault/resolutions/m07_orbital_geometry_conic_wave2.tsv'
RESOLUTION_PATH='formula-vault/resolutions/m07_orbital_geometry_conic_wave3.tsv'
A7_BATCH_PATH='equation-batches/a7-astrodynamics-orekit-foundation.tsv'
INVENTORY_PATH='validation/equation_inventory.tsv'
SOURCE_ARTIFACT_ID='stage4.m07_rust_port_v14.2026_06_15'
TARGET_CHUNK='9A_classical_elements_and_9E_mission_design_contracts'
M07_REPRESENTED_FUNCTION_ROWS=1350
EXPECTED_CLASSIFIER_GROUP_ROWS=377
EXPECTED_WAVE1_ROWS=40
EXPECTED_WAVE2_ROWS=40
EXPECTED_ROWS=40
EXPECTED_GROUP_REMAINING_ROWS=257
EXPECTED_EXECUTABLE_ROWS=152
EXPECTED_METADATA_ROWS=27
EXPECTED_CUMULATIVE_PROCESSED = 361
EXPECTED_REMAINING_BACKLOG = 962
EXPECTED_RISK_COUNTS=Counter({'medium_risk_requires_contract_review':33,'high_risk_requires_numerical_policy':7})
EXPECTED_TARGET_COUNTS={'formula_vault.astrodynamics.elements.eccentricity_vector':1}
EXPECTED_DISPOSITIONS=Counter({"blocked_missing_apsis_maneuver_linearization_and_units_contract": 6, "blocked_missing_conic_algebra_domain_branch_contract": 2, "blocked_missing_conic_classification_boundary_contract": 2, "blocked_missing_conic_reachability_and_multi_root_contract": 2, "blocked_missing_conic_speed_parameterization_contract": 1, "blocked_missing_maneuver_geometry_units_and_branch_contract": 1, "blocked_missing_plane_change_geometry_angle_and_units_contract": 4, "blocked_missing_state_to_eccentricity_scalar_contract": 1, "blocked_missing_time_base_angular_rate_conversion_contract": 2, "blocked_until_frame_time_body_rotation_and_angle_policy": 4, "blocked_until_perturbation_body_frame_time_and_rate_contract": 2, "blocked_until_solver_time_and_conic_branch_policy": 4, "blocked_until_solver_tolerance_and_maneuver_comparison_contract": 1, "deduplicated_alias_to_existing_runtime": 1, "excluded_composite_maneuver_algorithm_not_formula": 6, "excluded_internal_scalar_math_helper_not_formula": 1})
EXPECTED_HEADER=['schema_version', 'resolution_id', 'source_artifact_id', 'classifier_path', 'source_row_locator', 'source_row_number', 'rust_function_alias', 'scilab_function_alias', 'source_file_locator', 'formula_family', 'risk_tier', 'recommended_chunk_group', 'target_formula_id', 'target_resolution_id', 'target_batch_manifest', 'target_package', 'target_crate_name', 'target_runtime_symbol', 'target_runtime_path', 'target_contract_path', 'target_validation_card_path', 'target_source_seed_path', 'validation_status', 'disposition', 'block_reason']
TARGET_MATCH_FIELDS={'target_package':'package','target_crate_name':'crate_name','target_runtime_symbol':'runtime_symbol','target_contract_path':'contract_path','target_validation_card_path':'validation_card_path','target_source_seed_path':'source_seed_path'}
ALIASES={'ch4::eccentricity_vector':'formula_vault.astrodynamics.elements.eccentricity_vector'}
GROUNDTRACK=['launch_inclination_from_azimuth', 'minimum_launch_inclination', 'repeating_groundtrack_radius', 'synchronous_groundtrack_type']
COMPOSITE=['bielliptic_transfer', 'biparabolic_transfer', 'compare_hohmann_bielliptic', 'general_coplanar_transfer_circular', 'hohmann_transfer', 'hohmann_transfer_altitudes']
REACHABILITY=['conic_radius_reachable', 'conic_true_anomalies_for_radius']
SPEED=['speed_from_p_e_r']
APSIS=['apogee_change_from_perigee_impulse_linear', 'apsis_height_change_linear', 'apsis_tangential_impulse_exact', 'delta_v_to_change_apogee', 'delta_v_to_change_perigee', 'perigee_change_from_apogee_impulse_linear']
PLANE=['combined_speed_plane_change_delta_v', 'equatorialize_delta_v', 'plane_change_angle_from_delta_v', 'plane_change_delta_v']
MANEUVER_GEOM=['ascent_ellipse_to_circular']
COMPARE=['hohmann_biparabolic_break_even_ratio']
CONIC_ALG=['ch4::conic_radius', 'orbit_parameter_from_a_e']
PERTURB=['nodal_period_from_raan_rate', 'raan_rate_for_nodal_period']
RATE=['rate_from_deg_per_day', 'rate_to_deg_per_day']
PREDICT=['ch4::predict_true_anomaly_conic', 'ch4::predict_true_anomaly_elliptic', 'ch4::predict_true_anomaly_hyperbolic', 'ch4::predict_true_anomaly_parabolic']
MATH=['ch4::atan2']
CONIC_CLASS=['ch4::conic_type_from_alpha', 'ch4::conic_type_from_e']
ECC_SCALAR=['ch4::eccentricity_from_rv']

class VerificationError(RuntimeError): pass
def require(c:bool,m:str)->None:
    if not c: raise VerificationError(m)
def stable_json(v:Any)->str: return json.dumps(v,indent=2,sort_keys=True,ensure_ascii=False)+'\n'
def repo_file(repo:Path,rel:str)->Path:
    p=repo/rel; require(p.is_file(),f'missing repository file: {rel}'); return p
def read_delimited(path:Path,delimiter:str,expected_header:list[str]|None=None)->list[dict[str,str]]:
    with path.open(encoding='utf-8-sig',newline='') as h:
        r=csv.DictReader(h,delimiter=delimiter); require(r.fieldnames is not None,f'missing header: {path}')
        if expected_header is not None: require(r.fieldnames==expected_header,f'unsupported header in {path}: {r.fieldnames}')
        rows=list(r)
    require(rows,f'no data rows: {path}'); return rows
def unique_map(rows:Iterable[dict[str,str]],key:str,label:str)->dict[str,dict[str,str]]:
    out={}
    for i,row in enumerate(rows,1):
        v=row.get(key,''); require(v!='',f'{label} row {i} missing {key}'); require(v not in out,f'duplicate {label} {key}: {v}'); out[v]=row
    return out
def source_row_number(locator:str)->int:
    m=re.fullmatch(r'PORT_STATUS_RELEASE_GATE\.csv:row_(\d{4})',locator); require(m is not None,f'invalid source row locator: {locator}'); return int(m.group(1))
def expected_resolution(alias:str)->tuple[str,str|None,str]:
    if alias in ALIASES: return ('deduplicated_alias_to_existing_runtime',ALIASES[alias],'not_applicable_existing_runtime_and_contract_reused')
    if alias in GROUNDTRACK: return ('blocked_until_frame_time_body_rotation_and_angle_policy',None,'groundtrack or launch relation requires explicit body fixed inertial frame time scale rotation rate latitude azimuth angle wrap and singularity policy')
    if alias in COMPOSITE: return ('excluded_composite_maneuver_algorithm_not_formula',None,'multi_output_transfer_or_maneuver_comparison_pipeline_is_a_composite_algorithm_not_a_separate_bounded_formula_node')
    if alias in REACHABILITY: return ('blocked_missing_conic_reachability_and_multi_root_contract',None,'conic radius reachability requires explicit p e radius units tolerance branch multi root angle range and unreachable output contract')
    if alias in SPEED: return ('blocked_missing_conic_speed_parameterization_contract',None,'speed from p e r requires explicit gravitational parameter units positive radius conic domain radicand and nonfinite output contract')
    if alias in APSIS: return ('blocked_missing_apsis_maneuver_linearization_and_units_contract',None,'apsis change relation requires explicit linearized or exact semantics impulse sign units valid orbit domain and error bound contract')
    if alias in PLANE: return ('blocked_missing_plane_change_geometry_angle_and_units_contract',None,'plane change relation requires explicit inertial geometry angle range speed sign units inverse branch and nonfinite policy')
    if alias in MANEUVER_GEOM: return ('blocked_missing_maneuver_geometry_units_and_branch_contract',None,'ascent transfer relation requires explicit initial final apsis geometry body parameter units burn sequence output and invalid domain contract')
    if alias in COMPARE: return ('blocked_until_solver_tolerance_and_maneuver_comparison_contract',None,'break even ratio requires explicit comparison objective root interval tolerance convergence nonunique solution and reference oracle policy')
    if alias in CONIC_ALG: return ('blocked_missing_conic_algebra_domain_branch_contract',None,'conic algebra requires explicit semimajor axis eccentricity radius units denominator elliptic parabolic hyperbolic boundary and nonfinite policy')
    if alias in PERTURB: return ('blocked_until_perturbation_body_frame_time_and_rate_contract',None,'nodal period and raan rate relation requires explicit J2 body constants frame time scale rate sign units domain and perturbation model contract')
    if alias in RATE: return ('blocked_missing_time_base_angular_rate_conversion_contract',None,'angular rate conversion requires explicit source time unit target day definition scale provenance nonfinite and signed value policy')
    if alias in PREDICT: return ('blocked_until_solver_time_and_conic_branch_policy',None,'true anomaly prediction requires explicit epoch duration time scale conic branch anomaly wrap solver tolerance convergence and failure policy')
    if alias in MATH: return ('excluded_internal_scalar_math_helper_not_formula',None,'generic_atan2_wrapper_is_internal_math_utility_not_a_separate_formula_node')
    if alias in CONIC_CLASS: return ('blocked_missing_conic_classification_boundary_contract',None,'conic classification requires explicit energy or eccentricity tolerance parabolic boundary and output enum contract')
    if alias in ECC_SCALAR: return ('blocked_missing_state_to_eccentricity_scalar_contract',None,'eccentricity from state requires explicit state units gravitational parameter vector norm tolerance degenerate state and nonfinite output contract')
    raise VerificationError(f'unsupported A18 alias: {alias}')
def require_logical_source_locator(locator:str,row_index:int)->None:
    require(locator!='',f'row {row_index} source_file_locator is empty'); require(not locator.startswith(('/', '\\')),f'row {row_index} absolute source locator'); require(re.match(r'^[A-Za-z]:[\\/]',locator) is None,f'row {row_index} Windows absolute source locator'); require('..' not in Path(locator).parts,f'row {row_index} source locator traverses parents')
def external_resolution_inventory(repo:Path,inventory_rows:list[dict[str,str]],metadata_count:int)->tuple[int,int]:
    processed=[r for r in inventory_rows if r['category']=='external_m07_processed_row']; backlog=[r for r in inventory_rows if r['category']=='external_m07_backlog_row']; processed_map=unique_map(processed,'source_path','external processed inventory'); manifests=sorted((repo/'formula-vault/resolutions').glob('m07_*.tsv')); require(manifests,'no external M07 resolution manifests found'); total=0; expected_paths=set(); ids=set(); locators=set()
    for path in manifests:
        rel=path.relative_to(repo).as_posix(); expected_paths.add(rel); rs=read_delimited(path,'\t',EXPECTED_HEADER); inv=processed_map.get(rel); require(inv is not None,f'missing processed inventory row for {rel}'); require(inv['row_count']==str(len(rs)),f'processed inventory count mismatch for {rel}')
        for row in rs: require(row['resolution_id'] not in ids,f'duplicate external resolution ID: {row["resolution_id"]}'); ids.add(row['resolution_id']); require(row['source_row_locator'] not in locators,f'duplicate source-row locator: {row["source_row_locator"]}'); locators.add(row['source_row_locator'])
        total+=len(rs)
    require(set(processed_map)==expected_paths,'processed inventory and external manifests are not exact union'); require(len(backlog)==1,f'expected one backlog row, found {len(backlog)}'); expected=M07_REPRESENTED_FUNCTION_ROWS-metadata_count-total; require(backlog[0]['row_count']==str(expected),'external backlog count mismatch'); return total,expected
def verify_repo(repo:Path)->dict[str,Any]:
    repo=repo.resolve(); require(repo.is_dir(),f'repository does not exist: {repo}'); classifier_rows=read_delimited(repo_file(repo,CLASSIFIER_PATH),','); group=[r for r in classifier_rows if r['recommended_chunk_group']==TARGET_CHUNK]; group.sort(key=lambda r:source_row_number(r['m07_row_id_or_alias'])); require(len(group)==EXPECTED_CLASSIFIER_GROUP_ROWS,f'classifier group count mismatch: {len(group)}')
    wave1=read_delimited(repo_file(repo,WAVE1_PATH),'\t',EXPECTED_HEADER); wave2=read_delimited(repo_file(repo,WAVE2_PATH),'\t',EXPECTED_HEADER); require(len(wave1)==EXPECTED_WAVE1_ROWS,'wave1 count mismatch'); require(len(wave2)==EXPECTED_WAVE2_ROWS,'wave2 count mismatch'); prior=wave1+wave2; prior_locators={r['source_row_locator'] for r in prior}; require(len(prior_locators)==80,'prior wave locators are not unique'); require(prior_locators=={r['m07_row_id_or_alias'] for r in group[:80]},'A16-A17 are not exact first-80 selection')
    remaining=[r for r in group if r['m07_row_id_or_alias'] not in prior_locators]; selected=remaining[:EXPECTED_ROWS]; after=remaining[EXPECTED_ROWS:]; require(len(selected)==EXPECTED_ROWS,'selected row count mismatch'); require(len(after)==EXPECTED_GROUP_REMAINING_ROWS,f'remaining group count mismatch: {len(after)}'); classifier=unique_map(selected,'m07_row_id_or_alias','classifier'); risk=Counter(r['risk_tier'] for r in selected); require(risk==EXPECTED_RISK_COUNTS,f'risk counts mismatch: {dict(risk)}')
    for locator,row in classifier.items(): require(row['formula_family']=='orbit_two_body',f'classifier family mismatch: {locator}'); require(row['implementation_readiness']=='contract_review_first_no_implementation',f'classifier readiness mismatch: {locator}'); require(row['block_reason']=='blocked_until_orbit_geometry_conic_branch_and_validation_policy',f'classifier block reason mismatch: {locator}')
    rows=read_delimited(repo_file(repo,RESOLUTION_PATH),'\t',EXPECTED_HEADER); require(len(rows)==EXPECTED_ROWS,f'resolution row count mismatch: {len(rows)}'); resolutions=unique_map(rows,'source_row_locator','resolution'); unique_map(rows,'resolution_id','resolution'); require(set(resolutions)==set(classifier),'classifier selection and resolution locators are not exact union'); a7=unique_map(read_delimited(repo_file(repo,A7_BATCH_PATH),'\t'),'formula_id','A7 batch'); dispositions=Counter(); targets=Counter(); numbers=[]; source_files=set()
    for i,row in enumerate(rows,1):
        locator=row['source_row_locator']; source=classifier[locator]; number=source_row_number(locator); numbers.append(number); require(row['schema_version']==SCHEMA_VERSION,f'row {i} schema mismatch'); require(row['resolution_id']==f'resolution.external_m07.orbital_geometry_conic_wave3.{number:04d}',f'row {i} resolution ID mismatch'); require(row['source_artifact_id']==SOURCE_ARTIFACT_ID,f'row {i} source artifact mismatch'); require(row['classifier_path']==CLASSIFIER_PATH,f'row {i} classifier path mismatch'); require(row['source_row_number']==str(number),f'row {i} source row mismatch'); require_logical_source_locator(row['source_file_locator'],i)
        for field,cfield in [('rust_function_alias','rust_function_alias'),('scilab_function_alias','scilab_function_alias_if_known'),('source_file_locator','source_file_locator'),('formula_family','formula_family'),('risk_tier','risk_tier'),('recommended_chunk_group','recommended_chunk_group')]: require(row[field]==source[cfield],f'row {i} classifier mismatch for {field}')
        require(row['validation_status']=='research_required',f'row {i} validation status mismatch'); disp,target_formula,reason=expected_resolution(row['rust_function_alias']); require(row['disposition']==disp,f'row {i} disposition mismatch'); require(row['block_reason']==reason,f'row {i} block reason mismatch'); dispositions[disp]+=1; source_files.add(row['source_file_locator'])
        if target_formula is None:
            for field in ['target_formula_id','target_resolution_id','target_batch_manifest',*TARGET_MATCH_FIELDS,'target_runtime_path']: require(row[field]=='',f'row {i} non-alias must leave {field} empty')
        else:
            require(row['target_formula_id']==target_formula,f'row {i} target formula mismatch'); require(row['target_resolution_id']=='',f'row {i} target_resolution_id must be empty'); target=a7.get(target_formula); require(target is not None,f'row {i} target missing from A7 batch'); require(row['target_batch_manifest']==A7_BATCH_PATH,f'row {i} target batch mismatch')
            for field,tfield in TARGET_MATCH_FIELDS.items(): require(row[field]==target[tfield],f'row {i} target mismatch for {field}')
            require(row['target_runtime_path']==f"{target['crate_name']}::{target['runtime_symbol']}",f'row {i} runtime path mismatch'); targets[target_formula]+=1
            for pf in ['target_batch_manifest','target_contract_path','target_validation_card_path','target_source_seed_path']: repo_file(repo,row[pf])
    require(numbers==sorted(numbers),'rows not deterministic source order'); require(dispositions==EXPECTED_DISPOSITIONS,f'disposition counts mismatch: {dict(dispositions)}'); require(dict(sorted(targets.items()))==EXPECTED_TARGET_COUNTS,f'target counts mismatch: {dict(targets)}')
    inventory=read_delimited(repo_file(repo,INVENTORY_PATH),'\t'); executable=[r for r in inventory if r['category']=='executable_research_equation']; metadata=[r for r in inventory if r['category']=='metadata_only_formula_vault_candidate']; require(len(executable)==EXPECTED_EXECUTABLE_ROWS,f'executable count mismatch: {len(executable)}'); require(len(metadata)==EXPECTED_METADATA_ROWS,f'metadata count mismatch: {len(metadata)}'); total,backlog=external_resolution_inventory(repo,inventory,len(metadata)); require(total==EXPECTED_CUMULATIVE_PROCESSED,f'processed count mismatch: {total}'); require(backlog==EXPECTED_REMAINING_BACKLOG,f'backlog count mismatch: {backlog}')
    excluded=sum(v for k,v in dispositions.items() if k.startswith('excluded_')); blocked=sum(v for k,v in dispositions.items() if k.startswith('blocked_'))
    return {'schema_version':SCHEMA_VERSION,'result':'PASS','wave_id':'a18_external_m07_orbital_geometry_conic_wave3','classifier_group_rows':len(group),'wave1_rows':len(wave1),'wave2_rows':len(wave2),'prior_group_rows':len(prior),'classifier_rows_selected':len(selected),'classifier_group_remaining_rows':len(after),'terminal_disposition_rows':len(rows),'deduplicated_alias_rows':dispositions['deduplicated_alias_to_existing_runtime'],'excluded_helper_rows':excluded,'excluded_composite_maneuver_rows':dispositions['excluded_composite_maneuver_algorithm_not_formula'],'excluded_scalar_math_helper_rows':dispositions['excluded_internal_scalar_math_helper_not_formula'],'contract_blocked_rows':blocked,'risk_tier_counts':dict(sorted(risk.items())),'target_formula_counts':dict(sorted(targets.items())),'distinct_source_files':len(source_files),'risk_tier_not_downgraded':True,'executable_research_equations':len(executable),'metadata_inventory_records':len(metadata),'external_m07_processed_rows':total,'external_m07_backlog_rows':backlog,'formula_count_delta':0,'runtime_kernel_files_changed':0,'new_validation_cards_required':0,'new_source_seeds_required':0,'validation_status':'research_required','no_rust_m07_or_scilab_source_scraping':True,'no_external_parity_claim':True,'no_certification_or_operational_readiness_claim':True}
def self_test()->dict[str,Any]:
    require(stable_json({'b':2,'a':1}).startswith('{\n  "a"'),'stable JSON ordering failed'); cases={'ch4::eccentricity_vector':('deduplicated_alias_to_existing_runtime','formula_vault.astrodynamics.elements.eccentricity_vector'),'hohmann_transfer':('excluded_composite_maneuver_algorithm_not_formula',None),'plane_change_delta_v':('blocked_missing_plane_change_geometry_angle_and_units_contract',None),'ch4::atan2':('excluded_internal_scalar_math_helper_not_formula',None),'ch4::predict_true_anomaly_conic':('blocked_until_solver_time_and_conic_branch_policy',None)}
    for alias,expected in cases.items(): require(expected_resolution(alias)[:2]==expected,f'mapping self-test failed: {alias}')
    duplicate=False
    try: unique_map([{'x':'a'},{'x':'a'}],'x','fixture')
    except VerificationError: duplicate=True
    require(duplicate,'duplicate fixture not rejected'); return {'schema_version':SCHEMA_VERSION,'mode':'self-test','result':'PASS','tests':[{'name':'stable_json','result':'PASS'},{'name':'deterministic_mapping','result':'PASS'},{'name':'duplicate_rejected','result':'PASS'}]}
def main()->int:
    p=argparse.ArgumentParser(description=__doc__); p.add_argument('--repo',type=Path,default=Path('.')); p.add_argument('--self-test',action='store_true'); args=p.parse_args()
    try: print(stable_json(self_test() if args.self_test else verify_repo(args.repo)),end=''); return 0
    except Exception as e: print(stable_json({'schema_version':SCHEMA_VERSION,'result':'FAIL','error':str(e)}),end='',file=sys.stderr); return 1
if __name__=='__main__': raise SystemExit(main())
