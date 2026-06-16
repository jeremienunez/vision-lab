# Worker Contracts

Pydantic strict contracts live here:

- `job_payloads.py`
- `dataset_ingestion.py`
- `inference_payloads.py`
- `export_payloads.py`

Raw `dict` payloads are forbidden for jobs, exports, and inference.
