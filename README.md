# NOCTURNE-SDK

The **NOCTURNE-SDK** is a Python-native software development kit designed to facilitate the integration and utilization of the NOCTURNE ecosystem components. It provides a modular, secure, and efficient interface for interacting with services like the `TraumaEngine`, `ForgetMachine`, and `MirrorNetwork`.

## Overview

The SDK is organized into a series of packages, each encapsulating the functionality of a specific component of the NOCTURNE ecosystem. This modular architecture allows developers to use only the parts of the SDK they need, keeping their applications lean and efficient.

- **`trauma_engine`**: For ingesting, querying, and managing trauma records.
- **`forget_machine`**: For defining and enforcing data retention and "forgetting" policies.
- **`mirror_network`**: For peer-to-peer state synchronization.
- **`ai_ledger`**: For creating an immutable, auditable log of all operations.
- **And more...**

## Installation

To install the NOCTURNE-SDK, you can use `pip`:

```bash
pip install nocturne-sdk
```

*(Note: The SDK is not yet published to PyPI. This is the planned installation method.)*

## Quick Start

Here's a quick example of how to use the SDK to ingest a "trauma record":

1.  **Create a `config.yaml` file:**

    ```yaml
    trauma_engine:
      endpoint: "https://api.nocturne.io/trauma"
      api_key: "your-api-key"
    ```

2.  **Use the SDK in your Python application:**

    ```python
    from nocturne_sdk import load_config, TraumaClient
    from nocturne_sdk.trauma_engine.models import TraumaPayload

    # Load configuration
    cfg = load_config("config.yaml")

    # Initialize the client
    te_client = TraumaClient(cfg["trauma_engine"])

    # Create and ingest a payload
    payload = TraumaPayload(
        content="Sensitive user data",
        metadata={"source": "my-app"}
    )
    response = te_client.ingest(payload, meta={"user_id": "123"})

    print(f"Record ingested with ID: {response.record_id}")
    ```

This example demonstrates the basic workflow of configuring the SDK, initializing a client, and using it to interact with the NOCTURNE ecosystem. For more detailed examples, please see the `examples/` directory.
