from typing import Optional, Dict, Any
from datetime import datetime
from uuid import uuid4
from pydantic import BaseModel, Field

class TraumaPayload(BaseModel):
    id: str = Field(default_factory=lambda: str(uuid4()), alias="uuid")
    content: str
    metadata: Dict[str, Any]
    created_at: datetime = Field(default_factory=datetime.utcnow)

class IngestResponse(BaseModel):
    record_id: str
    status: str

class TraumaQuery(BaseModel):
    tag: Optional[str] = None
    start_date: Optional[datetime] = None
    end_date: Optional[datetime] = None
    # Add other query fields as needed

class TroubleRecord(BaseModel):
    id: str
    content: str
    metadata: Dict[str, Any]
    created_at: datetime

class ForgetProof(BaseModel):
    proof_id: str
    proof_blob: bytes
    merkle_root: str
    signature: str

class DeleteResponse(BaseModel):
    deleted_count: int
    status: str
