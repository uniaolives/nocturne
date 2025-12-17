from enum import Enum
from typing import List, Optional
from pydantic import BaseModel

class RetentionStrategy(str, Enum):
    TIME_BASED = "time_based"
    RISK_BASED = "risk_based"
    # Add other strategies as needed

class Policy(BaseModel):
    """
    Defines a forget policy for data retention and deletion.
    """
    retention_days: int
    strategy: RetentionStrategy
    exempt_tags: Optional[List[str]] = None

    def should_forget(self, record_tags: List[str], record_age_days: int) -> bool:
        """
        Evaluates whether a record should be forgotten based on this policy.
        """
        if self.exempt_tags and any(tag in self.exempt_tags for tag in record_tags):
            return False

        if self.strategy == RetentionStrategy.TIME_BASED:
            return record_age_days > self.retention_days

        # In a real implementation, other strategies would be handled here.
        return False
