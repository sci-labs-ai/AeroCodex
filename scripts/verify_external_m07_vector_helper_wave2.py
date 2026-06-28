#!/usr/bin/env python3
"""Verify A13 external M07 vector-helper Wave 2 terminal dispositions.

This standard-library-only verifier consumes classifier metadata and explicit
resolution records. It never opens or parses raw M07 or Scilab source text.
"""
from __future__ import annotations
import argparse, csv, json, re, sys
from collections import Counter
from pathlib import Path
from typing import Any, Iterable
SCHEMA_VERSION="aerocodex.external_m07_resolution.v1"
CLASSIFIER_PATH="docs/source_intake/m07_formula_family_classifier/low_risk_candidate_shortlist.csv"
WAVE1_PATH="formula-vault/resolutions/m07_vector_helper_wave1.tsv"
RESOLUTION_PATH="formula-vault/resolutions/m07_vector_helper_wave2.tsv"
M00_RUNTIME_LINKS_PATH="formula-vault/resolutions/m00_runtime_links.tsv"
INVENTORY_PATH="validation/equation_inventory.tsv"
SOURCE_ARTIFACT_ID="stage4.m07_rust_port_v14.2026_06_15"
TARGET_CHUNK="8D_helper_deduplication_then_low_risk_vector_contracts"
M07_REPRESENTED_FUNCTION_ROWS=1350
EXPECTED_CLASSIFIER_GROUP_ROWS=74
EXPECTED_WAVE1_ROWS=40
EXPECTED_ROWS=34
EXPECTED_EXECUTABLE_ROWS=152
EXPECTED_METADATA_ROWS=27
EXPECTED_CUMULATIVE_PROCESSED=1323
EXPECTED_REMAINING_BACKLOG=0
EXPECTED_TARGET_COUNTS={
 "formula_vault.m00.vector.angle":1,
 "formula_vault.m00.vector.cross":6,
 "formula_vault.m00.vector.dot":6,
 "formula_vault.m00.vector.norm":7,
 "formula_vault.m00.vector.unit":6,
}
EXPECTED_DISPOSITIONS=Counter({
 "deduplicated_alias_to_existing_runtime":26,
 "excluded_internal_shape_helper_not_formula":5,
 "blocked_missing_skew_symmetric_matrix_contract_and_runtime":2,
 "blocked_missing_true_anomaly_state_contract_and_runtime_mapping":1,
})
EXPECTED_HEADER="""schema_version resolution_id source_artifact_id classifier_path source_row_locator source_row_number rust_function_alias scilab_function_alias source_file_locator formula_family risk_tier recommended_chunk_group target_formula_id target_resolution_id target_batch_manifest target_package target_crate_name target_runtime_symbol target_runtime_path target_contract_path target_validation_card_path target_source_seed_path validation_status disposition block_reason""".split()
TARGET_MATCH_FIELDS={"target_resolution_id":"resolution_id","target_batch_manifest":"batch_manifest","target_package":"package","target_crate_name":"crate_name","target_runtime_symbol":"runtime_symbol","target_runtime_path":"runtime_path","target_contract_path":"contract_path","target_validation_card_path":"validation_card_path","target_source_seed_path":"source_seed_path"}
class VerificationError(RuntimeError): pass
def require(c:bool,m:str)->None:
 if not c: raise VerificationError(m)
def stable_json(v:Any)->str: return json.dumps(v,indent=2,sort_keys=True,ensure_ascii=False)+"\n"
def repo_file(repo:Path,rel:str)->Path:
 p=repo/rel; require(p.is_file(),f"missing repository file: {rel}"); return p
def read_delimited(path:Path,delimiter:str,expected_header:list[str]|None=None)->list[dict[str,str]]:
 with path.open(encoding="utf-8-sig",newline="") as h:
  r=csv.DictReader(h,delimiter=delimiter); require(r.fieldnames is not None,f"missing header: {path}")
  if expected_header is not None: require(r.fieldnames==expected_header,f"unsupported header in {path}: {r.fieldnames}")
  rows=list(r)
 require(rows,f"no data rows: {path}"); return rows
def unique_map(rows:Iterable[dict[str,str]],key:str,label:str)->dict[str,dict[str,str]]:
 out={}
 for i,row in enumerate(rows,1):
  value=row.get(key,""); require(value!="",f"{label} row {i} missing {key}"); require(value not in out,f"duplicate {label} {key}: {value}"); out[value]=row
 return out
def source_row_number(locator:str)->int:
 m=re.fullmatch(r"PORT_STATUS_RELEASE_GATE\.csv:row_(\d{4})",locator); require(m is not None,f"invalid source row locator: {locator}"); return int(m.group(1))
def expected_resolution(alias:str)->tuple[str,str|None,str]:
 low=alias.lower(); tail=low.split("::")[-1]
 if tail in {"col","columnize"} or low.endswith("_col"):
  return "excluded_internal_shape_helper_not_formula",None,"column_shape_normalization_is_internal_utility_not_public_formula"
 if tail.endswith("skew"):
  return "blocked_missing_skew_symmetric_matrix_contract_and_runtime",None,"skew_symmetric_cross_product_matrix_semantics_dimension_and_handedness_contract_required"
 if low=="ch7_true_anomaly_from_r_rdot":
  return "blocked_missing_true_anomaly_state_contract_and_runtime_mapping",None,"position_velocity_r_rdot_orbit_branch_and_angle_convention_contract_required"
 if tail.endswith("angle_between"): return "deduplicated_alias_to_existing_runtime","formula_vault.m00.vector.angle","not_applicable_existing_runtime_and_contract_reused"
 if tail.endswith("veccross") or tail.endswith("cross"): return "deduplicated_alias_to_existing_runtime","formula_vault.m00.vector.cross","not_applicable_existing_runtime_and_contract_reused"
 if tail.endswith("vecdot") or tail.endswith("dot"): return "deduplicated_alias_to_existing_runtime","formula_vault.m00.vector.dot","not_applicable_existing_runtime_and_contract_reused"
 if tail.endswith("vnorm") or tail.endswith("norm"): return "deduplicated_alias_to_existing_runtime","formula_vault.m00.vector.norm","not_applicable_existing_runtime_and_contract_reused"
 if tail.endswith("unit"): return "deduplicated_alias_to_existing_runtime","formula_vault.m00.vector.unit","not_applicable_existing_runtime_and_contract_reused"
 raise VerificationError(f"unsupported Wave 2 vector-helper alias: {alias}")
def require_logical_source_locator(locator:str,row_index:int)->None:
 require(locator!="",f"row {row_index} source_file_locator is empty"); require(not locator.startswith(("/","\\")),f"row {row_index} has absolute source locator"); require(re.match(r"^[A-Za-z]:[\\/]",locator) is None,f"row {row_index} has Windows-absolute source locator"); require(".." not in Path(locator).parts,f"row {row_index} source locator traverses parents")
def external_resolution_inventory(repo:Path,inventory_rows:list[dict[str,str]],metadata_count:int)->tuple[int,int]:
 processed=[r for r in inventory_rows if r["category"]=="external_m07_processed_row"]; backlog=[r for r in inventory_rows if r["category"]=="external_m07_backlog_row"]; processed_map=unique_map(processed,"source_path","external processed inventory")
 manifests=sorted((repo/"formula-vault/resolutions").glob("m07_*.tsv")); require(manifests,"no external M07 resolution manifests found")
 total=0; expected_paths=set()
 for path in manifests:
  rel=path.relative_to(repo).as_posix(); expected_paths.add(rel); rows=read_delimited(path,"\t",EXPECTED_HEADER); inv=processed_map.get(rel); require(inv is not None,f"missing processed inventory row for {rel}"); require(inv["row_count"]==str(len(rows)),f"processed inventory count mismatch for {rel}"); total+=len(rows)
 require(set(processed_map)==expected_paths,"processed inventory sources and external resolution manifests are not an exact union"); require(len(backlog)==1,f"expected one backlog aggregate inventory row, found {len(backlog)}")
 expected=M07_REPRESENTED_FUNCTION_ROWS-metadata_count-total; require(backlog[0]["row_count"]==str(expected),"external backlog count mismatch"); return total,expected
def verify_repo(repo:Path)->dict[str,Any]:
 repo=repo.resolve(); require(repo.is_dir(),f"repository does not exist: {repo}")
 classifier_rows=read_delimited(repo_file(repo,CLASSIFIER_PATH),","); group=[r for r in classifier_rows if r["recommended_chunk_group"]==TARGET_CHUNK]; group.sort(key=lambda r:source_row_number(r["m07_row_id_or_alias"])); require(len(group)==EXPECTED_CLASSIFIER_GROUP_ROWS,f"expected {EXPECTED_CLASSIFIER_GROUP_ROWS} group rows, found {len(group)}")
 wave1=read_delimited(repo_file(repo,WAVE1_PATH),"\t",EXPECTED_HEADER); require(len(wave1)==EXPECTED_WAVE1_ROWS,"Wave 1 row count mismatch")
 selected=group[EXPECTED_WAVE1_ROWS:]; require(len(selected)==EXPECTED_ROWS,f"expected {EXPECTED_ROWS} selected rows, found {len(selected)}"); classifier=unique_map(selected,"m07_row_id_or_alias","classifier")
 wave1_locators={r["source_row_locator"] for r in wave1}; require(wave1_locators.isdisjoint(classifier),"Wave 1 and Wave 2 classifier locators overlap"); require(wave1_locators|set(classifier)=={r["m07_row_id_or_alias"] for r in group},"Wave 1 and Wave 2 do not exactly cover the classifier group")
 for locator,row in classifier.items(): require(row["formula_family"]=="low_risk_vector_math",f"classifier family mismatch: {locator}"); require(row["risk_tier"]=="low_risk_candidate",f"classifier risk mismatch: {locator}"); require(row["contract_review_needed"]=="yes_standard_formula_vault_contract_and_source_locator_review",f"classifier review mismatch: {locator}")
 rows=read_delimited(repo_file(repo,RESOLUTION_PATH),"\t",EXPECTED_HEADER); require(len(rows)==EXPECTED_ROWS,f"expected {EXPECTED_ROWS} resolution rows, found {len(rows)}"); resolutions=unique_map(rows,"source_row_locator","resolution"); unique_map(rows,"resolution_id","resolution"); require(set(resolutions)==set(classifier),"classifier selection and resolution locators are not an exact union")
 runtime_links=unique_map(read_delimited(repo_file(repo,M00_RUNTIME_LINKS_PATH),"\t"),"formula_id","M00 runtime resolution")
 dispositions=Counter(); targets=Counter(); nums=[]; source_files=set()
 for i,row in enumerate(rows,1):
  locator=row["source_row_locator"]; source=classifier[locator]; n=source_row_number(locator); nums.append(n)
  require(row["schema_version"]==SCHEMA_VERSION,f"row {i} schema mismatch"); require(row["resolution_id"]==f"resolution.external_m07.vector_helper_wave2.{n:04d}",f"row {i} resolution ID mismatch"); require(row["source_artifact_id"]==SOURCE_ARTIFACT_ID,f"row {i} source artifact mismatch"); require(row["classifier_path"]==CLASSIFIER_PATH,f"row {i} classifier path mismatch"); require(row["source_row_number"]==str(n),f"row {i} source row number mismatch"); require_logical_source_locator(row["source_file_locator"],i)
  for field,cfield in [("rust_function_alias","rust_function_alias"),("scilab_function_alias","scilab_function_alias_if_known"),("source_file_locator","source_file_locator"),("formula_family","formula_family"),("risk_tier","risk_tier"),("recommended_chunk_group","recommended_chunk_group")]: require(row[field]==source[cfield],f"row {i} classifier mismatch for {field}")
  require(row["validation_status"]=="research_required",f"row {i} validation status mismatch"); disp,target,reason=expected_resolution(row["rust_function_alias"]); require(row["disposition"]==disp,f"row {i} disposition mismatch"); require(row["block_reason"]==reason,f"row {i} block reason mismatch"); dispositions[disp]+=1; source_files.add(row["source_file_locator"])
  if target is None:
   for field in ["target_formula_id",*TARGET_MATCH_FIELDS]: require(row[field]=="",f"row {i} non-alias must leave {field} empty")
  else:
   require(row["target_formula_id"]==target,f"row {i} target formula mismatch"); t=runtime_links.get(target); require(t is not None,f"row {i} target formula missing");
   for field,tfield in TARGET_MATCH_FIELDS.items(): require(row[field]==t[tfield],f"row {i} target mismatch for {field}")
   targets[target]+=1
   for pf in ["target_batch_manifest","target_contract_path","target_validation_card_path","target_source_seed_path"]: repo_file(repo,row[pf])
 require(nums==sorted(nums),"resolution rows are not deterministic source-row order"); require(dispositions==EXPECTED_DISPOSITIONS,f"disposition counts mismatch: {dict(dispositions)}"); require(dict(sorted(targets.items()))==EXPECTED_TARGET_COUNTS,f"target counts mismatch: {dict(targets)}")
 inventory=read_delimited(repo_file(repo,INVENTORY_PATH),"\t"); executable=[r for r in inventory if r["category"]=="executable_research_equation"]; metadata=[r for r in inventory if r["category"]=="metadata_only_formula_vault_candidate"]; require(len(executable)==EXPECTED_EXECUTABLE_ROWS,f"executable inventory mismatch: {len(executable)}"); require(len(metadata)==EXPECTED_METADATA_ROWS,f"metadata inventory mismatch: {len(metadata)}"); total,backlog=external_resolution_inventory(repo,inventory,len(metadata)); require(total==EXPECTED_CUMULATIVE_PROCESSED,f"cumulative processed mismatch: {total}"); require(backlog==EXPECTED_REMAINING_BACKLOG,f"remaining backlog mismatch: {backlog}")
 return {"schema_version":SCHEMA_VERSION,"result":"PASS","wave_id":"a13_external_m07_vector_helper_wave2","classifier_group_rows":len(group),"wave1_rows":len(wave1),"classifier_rows_selected":len(selected),"terminal_disposition_rows":len(rows),"deduplicated_alias_rows":dispositions["deduplicated_alias_to_existing_runtime"],"excluded_helper_rows":dispositions["excluded_internal_shape_helper_not_formula"],"contract_blocked_rows":dispositions["blocked_missing_skew_symmetric_matrix_contract_and_runtime"]+dispositions["blocked_missing_true_anomaly_state_contract_and_runtime_mapping"],"skew_contract_blocked_rows":dispositions["blocked_missing_skew_symmetric_matrix_contract_and_runtime"],"true_anomaly_contract_blocked_rows":dispositions["blocked_missing_true_anomaly_state_contract_and_runtime_mapping"],"target_formula_counts":dict(sorted(targets.items())),"distinct_source_files":len(source_files),"vector_helper_group_complete":True,"vector_helper_group_terminal_rows":len(wave1)+len(rows),"executable_research_equations":len(executable),"metadata_inventory_records":len(metadata),"external_m07_processed_rows":total,"external_m07_backlog_rows":backlog,"formula_count_delta":0,"runtime_kernel_files_changed":0,"new_validation_cards_required":0,"new_source_seeds_required":0,"validation_status":"research_required","no_rust_or_scilab_source_scraping":True,"no_external_parity_claim":True,"no_certification_or_operational_readiness_claim":True}
def self_test()->dict[str,Any]:
 tests=[]; require(stable_json({"b":2,"a":1}).startswith('{\n  "a"'),"stable JSON ordering failed"); tests.append({"name":"stable_json","result":"PASS"})
 mappings={"fb_cross":("deduplicated_alias_to_existing_runtime","formula_vault.m00.vector.cross"),"ga_angle_between":("deduplicated_alias_to_existing_runtime","formula_vault.m00.vector.angle"),"ac_col":("excluded_internal_shape_helper_not_formula",None),"ga_skew":("blocked_missing_skew_symmetric_matrix_contract_and_runtime",None),"ch7_true_anomaly_from_r_rdot":("blocked_missing_true_anomaly_state_contract_and_runtime_mapping",None)}
 for alias,expected in mappings.items(): require(expected_resolution(alias)[:2]==expected,f"mapping self-test failed: {alias}")
 tests.append({"name":"deterministic_alias_and_block_mapping","result":"PASS"}); duplicate=False
 try: unique_map([{"x":"a"},{"x":"a"}],"x","fixture")
 except VerificationError: duplicate=True
 require(duplicate,"duplicate fixture not rejected"); tests.append({"name":"duplicate_rejected","result":"PASS"}); return {"schema_version":SCHEMA_VERSION,"mode":"self-test","result":"PASS","tests":tests}
def parse_args()->argparse.Namespace:
 p=argparse.ArgumentParser(description=__doc__); p.add_argument("--repo",type=Path,default=Path(".")); p.add_argument("--self-test",action="store_true"); return p.parse_args()
def main()->int:
 args=parse_args()
 try: print(stable_json(self_test() if args.self_test else verify_repo(args.repo)),end=""); return 0
 except Exception as e: print(stable_json({"schema_version":SCHEMA_VERSION,"result":"FAIL","error":str(e)}),end="",file=sys.stderr); return 1
if __name__=="__main__": raise SystemExit(main())
