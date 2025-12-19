import React, { useState } from 'react';
import { Shield, Zap, CheckCircle, XCircle, AlertTriangle, FileText, Database, Cpu } from 'lucide-react';

// ============================================================================
// CRYPTO UTILITIES (Simplified for Demo)
// ============================================================================

const sha256 = async (data) => {
  const encoder = new TextEncoder();
  const buffer = await crypto.subtle.digest('SHA-256', encoder.encode(data));
  return Array.from(new Uint8Array(buffer))
    .map(b => b.toString(16).padStart(2, '0'))
    .join('');
};

const canonicalJSON = (obj) => {
  return JSON.stringify(obj, Object.keys(obj).sort(), null, 0);
};

const sha256Canonical = async (obj) => {
  return sha256(canonicalJSON(obj));
};

// ============================================================================
// INTEGRATED SYSTEM: NoctHub + ZK-PoS + Hardware Registry
// ============================================================================

class IntegratedNoctHub {
  constructor() {
    this.hardwareRegistry = new Map();
    this.revocationList = new Map();
    this.secretRemovalProofs = [];
    this.zkPosProofs = [];
    this.goldenVectors = [];
  }

  // I1: Register hardware with tier
  async registerHardware(sensorId, tier, manufacturer) {
    const pubKey = `pub_${sensorId}`;
    const firmwareHash = await sha256(`firmware_${sensorId}_v1`);
    
    this.hardwareRegistry.set(sensorId, {
      sensorId,
      pubKey,
      tier,
      manufacturer,
      firmwareHash,
      registeredAt: Date.now(),
      revoked: false
    });
    
    return { sensorId, pubKey, tier };
  }

  // H3: Revoke sensor
  async revokeSensor(sensorId, reason) {
    const sensor = this.hardwareRegistry.get(sensorId);
    if (!sensor) throw new Error(`Sensor ${sensorId} not found`);
    
    sensor.revoked = true;
    sensor.revokedAt = Date.now();
    
    this.revocationList.set(sensorId, {
      sensorId,
      revokedAt: sensor.revokedAt,
      reason
    });
  }

  // Z1 + I3: Verify ZK-PoS proof
  async verifyZKPoSProof(proof, blockHeader) {
    const results = {
      z1_temporal: false,
      i3_hardware: false,
      overall: false,
      details: []
    };

    // Z1: Temporal binding
    const expectedNonce = await sha256Canonical(blockHeader);
    if (proof.nonce !== expectedNonce) {
      results.details.push({
        invariant: 'Z1',
        status: 'FAIL',
        message: 'Nonce mismatch - not bound to current block'
      });
      return results;
    }

    const timeDiff = Math.abs(proof.timestamp - blockHeader.timestamp);
    if (timeDiff > 300) {
      results.details.push({
        invariant: 'Z1',
        status: 'FAIL',
        message: `Time drift ${timeDiff}s exceeds 300s`
      });
      return results;
    }

    results.z1_temporal = true;
    results.details.push({
      invariant: 'Z1',
      status: 'PASS',
      message: 'Temporal binding valid'
    });

    // I3: Hardware registry check
    const sensor = this.hardwareRegistry.get(proof.sensorId);
    if (!sensor) {
      results.details.push({
        invariant: 'I3',
        status: 'FAIL',
        message: 'Sensor not registered'
      });
      return results;
    }

    if (sensor.revoked) {
      results.details.push({
        invariant: 'I3',
        status: 'FAIL',
        message: `Sensor revoked at ${new Date(sensor.revokedAt).toISOString()}`
      });
      return results;
    }

    results.i3_hardware = true;
    results.details.push({
      invariant: 'I3',
      status: 'PASS',
      message: `Sensor valid (tier: ${sensor.tier})`
    });

    results.overall = results.z1_temporal && results.i3_hardware;
    return results;
  }

  // I4: Create data vector with tier propagation
  async createDataVector(sensorId, value) {
    const sensor = this.hardwareRegistry.get(sensorId);
    if (!sensor) throw new Error('Sensor not registered');

    return {
      vectorId: `vec_${Date.now()}`,
      origin: 'MEASUREMENT',
      tier: sensor.tier, // I4: Tier propagation
      sensorId,
      value,
      timestamp: Date.now()
    };
  }

  // I3: Verify Secret Removal Proof with hardware validation
  async verifySRP(srp) {
    const results = {
      valid: true,
      errors: []
    };

    // Check all signatures are from valid hardware
    for (const sig of srp.pog.signatures) {
      const sensor = this.hardwareRegistry.get(sig.sensorId);
      
      if (!sensor) {
        results.valid = false;
        results.errors.push(`I3: Unknown sensor ${sig.sensorId} in SRP`);
      } else if (sensor.revoked) {
        results.valid = false;
        results.errors.push(`I3: Revoked sensor ${sig.sensorId} in SRP`);
      } else if (sensor.tier === 'L0') {
        results.valid = false;
        results.errors.push(`I3: L0 sensor ${sig.sensorId} insufficient for SRP`);
      }
    }

    return results;
  }
}

// ============================================================================
// GOLDEN VECTORS (I1: Cross-system validation)
// ============================================================================

const GOLDEN_VECTORS = [
  {
    id: 'GV-ZK-001',
    name: 'Valid ZK-PoS Proof (L3 Sensor)',
    type: 'zkpos',
    proof: {
      sensorId: 'SENS-L3-001',
      nonce: '', // Will be computed
      timestamp: 1700000100,
      commitment: '0xabcd1234'
    },
    blockHeader: {
      height: 1000,
      timestamp: 1700000000,
      prevHash: '0x000'
    },
    expected: 'VALID'
  },
  {
    id: 'GV-ZK-002',
    name: 'Revoked Sensor Proof',
    type: 'zkpos',
    proof: {
      sensorId: 'SENS-REVOKED',
      nonce: '',
      timestamp: 1700000100,
      commitment: '0xabcd1234'
    },
    blockHeader: {
      height: 1000,
      timestamp: 1700000000,
      prevHash: '0x000'
    },
    expected: 'INVALID'
  },
  {
    id: 'GV-I4-001',
    name: 'Tier Propagation L3→Vector',
    type: 'tier_propagation',
    sensorId: 'SENS-L3-001',
    expectedTier: 'L3'
  },
  {
    id: 'GV-SRP-001',
    name: 'SRP with Valid Hardware',
    type: 'srp',
    srp: {
      secretScope: 'test-secret',
      pog: {
        signatures: [
          { sensorId: 'SENS-L3-001' },
          { sensorId: 'SENS-L2-001' }
        ]
      }
    },
    expected: 'VALID'
  }
];

// ============================================================================
// REACT UI
// ============================================================================

export default function NoctHubIntegration() {
  const [system] = useState(() => new IntegratedNoctHub());
  const [testResults, setTestResults] = useState([]);
  const [activeTab, setActiveTab] = useState('overview');
  const [proofDetails, setProofDetails] = useState(null);

  // Initialize test data
  const initializeSystem = async () => {
    // Register L3 sensor
    await system.registerHardware('SENS-L3-001', 'L3', 'Silicon-Alpha');
    
    // Register L2 sensor
    await system.registerHardware('SENS-L2-001', 'L2', 'Beta-Sensors');
    
    // Register and revoke sensor
    await system.registerHardware('SENS-REVOKED', 'L2', 'Beta-Sensors');
    await system.revokeSensor('SENS-REVOKED', 'Physical compromise detected');
  };

  // Run golden vector tests
  const runGoldenTests = async () => {
    await initializeSystem();
    const results = [];

    for (const gv of GOLDEN_VECTORS) {
      let result = {
        id: gv.id,
        name: gv.name,
        type: gv.type,
        expected: gv.expected,
        actual: '',
        passed: false,
        details: null
      };

      try {
        if (gv.type === 'zkpos') {
          // Compute nonce
          gv.proof.nonce = await sha256Canonical(gv.blockHeader);
          
          // Verify proof
          const verification = await system.verifyZKPoSProof(gv.proof, gv.blockHeader);
          result.actual = verification.overall ? 'VALID' : 'INVALID';
          result.passed = result.actual === gv.expected;
          result.details = verification.details;
        } else if (gv.type === 'tier_propagation') {
          // Test I4: Tier propagation
          const vector = await system.createDataVector(gv.sensorId, 42.5);
          result.actual = vector.tier;
          result.passed = vector.tier === gv.expectedTier;
          result.details = [{ message: `Vector tier: ${vector.tier}` }];
        } else if (gv.type === 'srp') {
          // Test I3: SRP validation
          const verification = await system.verifySRP(gv.srp);
          result.actual = verification.valid ? 'VALID' : 'INVALID';
          result.passed = result.actual === gv.expected;
          result.details = verification.errors.map(e => ({ message: e }));
        }
      } catch (e) {
        result.actual = 'ERROR';
        result.passed = false;
        result.details = [{ message: e.message }];
      }

      results.push(result);
    }

    setTestResults(results);
  };

  // Test specific proof
  const testProof = async (sensorId) => {
    await initializeSystem();

    const blockHeader = {
      height: 1000,
      timestamp: Math.floor(Date.now() / 1000),
      prevHash: '0xabc123'
    };

    const nonce = await sha256Canonical(blockHeader);

    const proof = {
      sensorId,
      nonce,
      timestamp: blockHeader.timestamp + 50,
      commitment: '0xtest1234'
    };

    const verification = await system.verifyZKPoSProof(proof, blockHeader);
    setProofDetails({
      proof,
      blockHeader,
      verification
    });
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-blue-900 to-slate-900 text-white p-6">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <div className="flex items-center gap-3 mb-2">
            <Shield className="w-10 h-10 text-cyan-400" />
            <Zap className="w-8 h-8 text-yellow-400" />
            <Database className="w-8 h-8 text-green-400" />
            <h1 className="text-3xl font-bold">NoctHub + ZK-PoS Integration</h1>
          </div>
          <p className="text-slate-400">
            Distributed Memory with Verifiable Hardware Sensing
          </p>
        </div>

        {/* Tabs */}
        <div className="flex gap-2 mb-6 border-b border-slate-700">
          {['overview', 'golden-tests', 'live-test', 'audit'].map(tab => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={`px-4 py-2 font-medium transition-colors ${
                activeTab === tab
                  ? 'border-b-2 border-cyan-400 text-cyan-400'
                  : 'text-slate-400 hover:text-white'
              }`}
            >
              {tab.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ')}
            </button>
          ))}
        </div>

        {/* Overview */}
        {activeTab === 'overview' && (
          <div className="space-y-6">
            <div className="grid grid-cols-3 gap-4">
              <div className="bg-slate-800 rounded-lg p-6 border border-cyan-500">
                <div className="flex items-center gap-2 mb-2">
                  <Shield className="w-5 h-5 text-cyan-400" />
                  <h3 className="font-bold">NoctHub Core</h3>
                </div>
                <p className="text-sm text-slate-400">Secret Removal Proofs</p>
                <p className="text-sm text-slate-400">Merkle Tree State</p>
                <p className="text-sm text-slate-400">BLS Signatures</p>
              </div>

              <div className="bg-slate-800 rounded-lg p-6 border border-yellow-500">
                <div className="flex items-center gap-2 mb-2">
                  <Zap className="w-5 h-5 text-yellow-400" />
                  <h3 className="font-bold">ZK-PoS</h3>
                </div>
                <p className="text-sm text-slate-400">Zero-Knowledge Proofs</p>
                <p className="text-sm text-slate-400">Temporal Binding (Z1)</p>
                <p className="text-sm text-slate-400">Hardware Attestation</p>
              </div>

              <div className="bg-slate-800 rounded-lg p-6 border border-green-500">
                <div className="flex items-center gap-2 mb-2">
                  <Database className="w-5 h-5 text-green-400" />
                  <h3 className="font-bold">Hardware Registry</h3>
                </div>
                <p className="text-sm text-slate-400">Tier Classification (L0-L3)</p>
                <p className="text-sm text-slate-400">Revocation List (H3)</p>
                <p className="text-sm text-slate-400">Certificate Chains (H2)</p>
              </div>
            </div>

            <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
              <h2 className="text-xl font-bold mb-4">Integration Invariants</h2>
              <div className="space-y-3">
                {[
                  {
                    id: 'I1',
                    name: 'Golden Vector Consistency',
                    desc: 'All systems agree on all test vectors'
                  },
                  {
                    id: 'I3',
                    name: 'Cross-Validation',
                    desc: 'SRP signatures validated against hardware registry'
                  },
                  {
                    id: 'I4',
                    name: 'Tier Propagation',
                    desc: 'Data vectors inherit sensor tier (L0-L3)'
                  },
                  {
                    id: 'Z1',
                    name: 'Temporal Binding',
                    desc: 'Proofs anchored to blockchain time'
                  }
                ].map(inv => (
                  <div key={inv.id} className="flex items-start gap-3 bg-slate-900 rounded p-3">
                    <div className="font-mono text-cyan-400 font-bold">{inv.id}</div>
                    <div>
                      <div className="font-medium">{inv.name}</div>
                      <div className="text-sm text-slate-400">{inv.desc}</div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* Golden Tests */}
        {activeTab === 'golden-tests' && (
          <div className="space-y-6">
            <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
              <div className="flex items-center justify-between mb-4">
                <h2 className="text-xl font-bold">Golden Vector Test Suite</h2>
                <button
                  onClick={runGoldenTests}
                  className="flex items-center gap-2 px-4 py-2 bg-cyan-600 hover:bg-cyan-500 rounded transition-colors"
                >
                  <Cpu className="w-4 h-4" />
                  Run All Tests
                </button>
              </div>

              {testResults.length > 0 && (
                <div className="space-y-3">
                  {testResults.map(result => (
                    <div
                      key={result.id}
                      className={`p-4 rounded border ${
                        result.passed
                          ? 'bg-green-900/20 border-green-500'
                          : 'bg-red-900/20 border-red-500'
                      }`}
                    >
                      <div className="flex items-center justify-between mb-2">
                        <div className="flex items-center gap-2">
                          {result.passed ? (
                            <CheckCircle className="w-5 h-5 text-green-400" />
                          ) : (
                            <XCircle className="w-5 h-5 text-red-400" />
                          )}
                          <span className="font-medium">{result.name}</span>
                          <span className="text-xs font-mono text-slate-400">{result.id}</span>
                        </div>
                        <div className="text-sm">
                          <span className="text-slate-400">Expected:</span>{' '}
                          <span className="font-mono">{result.expected}</span>
                          {' | '}
                          <span className="text-slate-400">Actual:</span>{' '}
                          <span className="font-mono">{result.actual}</span>
                        </div>
                      </div>
                      
                      {result.details && result.details.length > 0 && (
                        <div className="mt-2 space-y-1">
                          {result.details.map((d, i) => (
                            <div key={i} className="text-xs font-mono text-slate-400 flex items-start gap-2">
                              {d.invariant && <span className="text-cyan-400">[{d.invariant}]</span>}
                              {d.status && (
                                <span className={d.status === 'PASS' ? 'text-green-400' : 'text-red-400'}>
                                  {d.status}
                                </span>
                              )}
                              <span>{d.message}</span>
                            </div>
                          ))}
                        </div>
                      )}
                    </div>
                  ))}

                  <div className="mt-6 p-4 bg-slate-900 rounded border border-slate-700">
                    <div className="text-lg font-bold mb-2">Test Summary</div>
                    <div className="grid grid-cols-3 gap-4 text-center">
                      <div>
                        <div className="text-2xl font-bold text-cyan-400">
                          {testResults.length}
                        </div>
                        <div className="text-sm text-slate-400">Total</div>
                      </div>
                      <div>
                        <div className="text-2xl font-bold text-green-400">
                          {testResults.filter(r => r.passed).length}
                        </div>
                        <div className="text-sm text-slate-400">Passed</div>
                      </div>
                      <div>
                        <div className="text-2xl font-bold text-red-400">
                          {testResults.filter(r => !r.passed).length}
                        </div>
                        <div className="text-sm text-slate-400">Failed</div>
                      </div>
                    </div>
                  </div>
                </div>
              )}
            </div>
          </div>
        )}

        {/* Live Test */}
        {activeTab === 'live-test' && (
          <div className="space-y-6">
            <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
              <h2 className="text-xl font-bold mb-4">Test ZK-PoS Proof</h2>
              <div className="flex gap-2 mb-4">
                {['SENS-L3-001', 'SENS-L2-001', 'SENS-REVOKED'].map(id => (
                  <button
                    key={id}
                    onClick={() => testProof(id)}
                    className="px-4 py-2 bg-cyan-600 hover:bg-cyan-500 rounded transition-colors"
                  >
                    {id}
                  </button>
                ))}
              </div>

              {proofDetails && (
                <div className="space-y-4">
                  <div className="bg-slate-900 rounded p-4">
                    <div className="font-bold mb-2">Proof Details</div>
                    <div className="text-xs font-mono text-slate-400 space-y-1">
                      <div>Sensor: {proofDetails.proof.sensorId}</div>
                      <div>Nonce: {proofDetails.proof.nonce.slice(0, 16)}...</div>
                      <div>Timestamp: {proofDetails.proof.timestamp}</div>
                    </div>
                  </div>

                  <div className={`p-4 rounded border ${
                    proofDetails.verification.overall
                      ? 'bg-green-900/20 border-green-500'
                      : 'bg-red-900/20 border-red-500'
                  }`}>
                    <div className="flex items-center gap-2 mb-3">
                      {proofDetails.verification.overall ? (
                        <CheckCircle className="w-5 h-5 text-green-400" />
                      ) : (
                        <XCircle className="w-5 h-5 text-red-400" />
                      )}
                      <span className="font-bold">
                        {proofDetails.verification.overall ? 'PROOF VALID' : 'PROOF INVALID'}
                      </span>
                    </div>
                    
                    <div className="space-y-2">
                      {proofDetails.verification.details.map((d, i) => (
                        <div key={i} className="text-sm flex items-start gap-2">
                          <span className="font-mono text-cyan-400">[{d.invariant}]</span>
                          <span className={d.status === 'PASS' ? 'text-green-400' : 'text-red-400'}>
                            {d.status}:
                          </span>
                          <span className="text-slate-300">{d.message}</span>
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              )}
            </div>
          </div>
        )}

        {/* Audit */}
        {activeTab === 'audit' && (
          <div className="bg-slate-800 rounded-lg p-6 border border-slate-700">
            <h2 className="text-xl font-bold mb-4 flex items-center gap-2">
              <FileText className="w-5 h-5 text-amber-400" />
              How to Audit This Integration
            </h2>
            <div className="space-y-4 text-sm">
              <div className="bg-slate-900 rounded p-4">
                <div className="font-bold text-cyan-400 mb-2">I1 — Golden Vector Consistency</div>
                <div className="text-slate-300 mb-2">
                  ✓ Check: All test results match expected values
                </div>
                <div className="text-slate-400">
                  Run: Click "Run All Tests" in Golden Tests tab
                </div>
                <div className="text-slate-400">
                  Verify: All 4 vectors pass (3 valid, 1 invalid expected)
                </div>
              </div>

              <div className="bg-slate-900 rounded p-4">
                <div className="font-bold text-cyan-400 mb-2">I3 — Cross-Validation</div>
                <div className="text-slate-300 mb-2">
                  ✓ Check: <code className="text-amber-400">verifySRP()</code> function
                </div>
                <div className="text-slate-400 space-y-1">
                  <div>1. Verify all signatures come from registered sensors</div>
                  <div>2. Check no revoked sensors in SRP</div>
                  <div>3. Ensure minimum tier requirement (no L0 in critical proofs)</div>
                </div>
              </div>

              <div className="bg-slate-900 rounded p-4">
                <div className="font-bold text-cyan-400 mb-2">I4 — Tier Propagation</div>
                <div className="text-slate-300 mb-2">
                  ✓ Check: <code className="text-amber-400">createDataVector()</code> function
                </div>
                <div className="text-slate-400 space-y-1">
                  <div>1. Data vector inherits sensor tier</div>
                  <div>2. Test GV-I4-001: L3 sensor → L3 vector</div>
                  <div>3. Verify origin=MEASUREMENT always has tier</div>
                </div>
              </div>

              <div className="bg-slate-900 rounded p-4">
                <div className="font-bold text-cyan-400 mb-2">Z1 — Temporal Binding</div>
                <div className="text-slate-300 mb-2">
                  ✓ Check: <code className="text-amber-400">verifyZKPoSProof()</code> function
                </div>
                <div className="text-slate-400 space-y-1">
                  <div>1. Nonce computed from canonical block header</div>
                  <div>2. Time drift checked (max 300 seconds)</div>
                  <div>3. Test with SENS-L3-001 in Live Test tab</div>
                </div>
              </div>

              <div className="bg-amber-900/20 border border-amber-500 rounded p-4">
                <div className="font-bold text-amber-400 mb-2">⚠️ PRODUCTION REQUIREMENTS</div>
                <div className="text-slate-300 space-y-1 text-xs">
                  <div>• Replace simplified crypto with real BLS12-381</div>
                  <div>• Implement actual ZK-SNARK verifier (not simulated)</div>
                  <div>• Add Byzantine fault tolerance for hardware registry</div>
                  <div>• Deploy Python auditor alongside Rust (A4 divergence detection)</div>
                  <div>• Conduct formal verification of circuit constraints</div>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}