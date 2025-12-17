from nocturne_sdk.config import load_config
from nocturne_sdk.trauma_engine.client import TraumaClient
from nocturne_sdk.trauma_engine.models import TraumaPayload
from nocturne_sdk.forget_machine.policy import Policy, RetentionStrategy

# In a real app, other clients would be initialized here as well.
# from nocturne_sdk.forget_machine import ForgetMachine
# from nocturne_sdk.mirror_network import MirrorNetwork
# from nocturne_sdk.ai_ledger import AILedger
# from nocturne_sdk.scroll_publisher import ScrollPublisher
# from nocturne_sdk.dam import DAM


def main():
    """
    Quick-start example for the NOCTURNE-SDK.
    """
    print("🚀 NOCTURNE SDK Quickstart")

    # 1. Load configuration
    try:
        cfg = load_config("examples/config.yaml")
        print("✓ Configuration loaded")
    except FileNotFoundError:
        print("❌ Configuration file not found. Please create 'examples/config.yaml'.")
        return


    # 2. Initialize clients (only TraumaClient is functional for now)
    te_client = TraumaClient(cfg["trauma_engine"])
    # In a real implementation, you would also initialize:
    # fm = ForgetMachine(cfg["forget_machine"])
    # mn = MirrorNetwork(cfg["mirror_network"])
    # ledger = AILedger()
    # scroll = ScrollPublisher(cfg["scroll_publisher"])
    # dam = DAM(cfg["dam"])
    print("✓ Clients initialized")


    # 3. Ingest a trauma record
    print("\n📥 Ingesting data...")
    payload = TraumaPayload(
        content="Paciente X - diagnóstico Y",
        metadata={"source": "app_clinica"}
    )
    ingest_response = te_client.ingest(
        payload=payload,
        meta={"user": "dr_smith"}
    )
    print(f"  ✓ Record ingested with ID: {ingest_response.record_id}")


    # 4. Schedule forgetting according to a policy
    # The ForgetMachine is not fully implemented, so this is a demonstration of the policy object.
    policy = Policy(retention_days=30, strategy=RetentionStrategy.TIME_BASED)
    print(f"\n🗑️  Scheduling record for forgetting based on policy: {policy.model_dump_json()}")
    # In a real app, you would call:
    # fm.schedule_forget(record_id, policy="default")


    print("\n✨ Quickstart complete!")


if __name__ == "__main__":
    main()
