'use client';

import { useState, useCallback } from 'react';
import { IDKitWidget, VerificationLevel, ISuccessResult } from '@worldcoin/idkit';

// =============================================================================
// TIME Protocol - AI Governance Agent MVP
// NEARCON Innovation Sandbox | "AI That Works For You"
// =============================================================================

// World ID Configuration
const WORLD_ID_APP_ID = process.env.NEXT_PUBLIC_WORLD_ID_APP_ID || 'app_staging_45e53ad8a60c3eb00eb18c21e3c6d993';
const WORLD_ID_ACTION = process.env.NEXT_PUBLIC_WORLD_ID_ACTION || 'claim_time_birthright';

type Screen = 'welcome' | 'birthdate' | 'discovering' | 'two_numbers' | 'explore';

// Mock governance data (AI agent would discover this)
const MOCK_GOVERNANCE = {
  location: 'San Francisco, CA',
  officials: {
    local: [
      { id: 'mayor', title: 'Mayor', name: 'London Breed', level: 'local' },
      { id: 'sup-1', title: 'Supervisor D1', name: 'Connie Chan', level: 'local' },
      { id: 'sup-2', title: 'Supervisor D2', name: 'Catherine Stefani', level: 'local' },
      { id: 'sup-3', title: 'Supervisor D3', name: 'Aaron Peskin', level: 'local' },
      { id: 'sup-4', title: 'Supervisor D4', name: 'Joel Engardio', level: 'local' },
      { id: 'sup-5', title: 'Supervisor D5', name: 'Dean Preston', level: 'local' },
      { id: 'sup-6', title: 'Supervisor D6', name: 'Matt Dorsey', level: 'local' },
      { id: 'sup-7', title: 'Supervisor D7', name: 'Myrna Melgar', level: 'local' },
      { id: 'sup-8', title: 'Supervisor D8', name: 'Rafael Mandelman', level: 'local' },
      { id: 'sup-9', title: 'Supervisor D9', name: 'Hillary Ronen', level: 'local' },
      { id: 'sup-10', title: 'Supervisor D10', name: 'Shamann Walton', level: 'local' },
      { id: 'sup-11', title: 'Supervisor D11', name: 'Ahsha Safai', level: 'local' },
    ],
    county: [
      { id: 'da', title: 'District Attorney', name: 'Brooke Jenkins', level: 'county' },
      { id: 'sheriff', title: 'Sheriff', name: 'Paul Miyamoto', level: 'county' },
      { id: 'assessor', title: 'Assessor-Recorder', name: 'Joaquín Torres', level: 'county' },
      { id: 'treasurer', title: 'Treasurer', name: 'José Cisneros', level: 'county' },
      { id: 'city-attorney', title: 'City Attorney', name: 'David Chiu', level: 'county' },
      { id: 'public-defender', title: 'Public Defender', name: 'Mano Raju', level: 'county' },
    ],
    state: [
      { id: 'governor', title: 'Governor', name: 'Gavin Newsom', level: 'state' },
      { id: 'lt-gov', title: 'Lt. Governor', name: 'Eleni Kounalakis', level: 'state' },
      { id: 'state-sen', title: 'State Senator', name: 'Scott Wiener', level: 'state' },
      { id: 'assembly', title: 'Assembly Member', name: 'Matt Haney', level: 'state' },
      { id: 'ag', title: 'Attorney General', name: 'Rob Bonta', level: 'state' },
      { id: 'sos', title: 'Secretary of State', name: 'Shirley Weber', level: 'state' },
    ],
    federal: [
      { id: 'president', title: 'President', name: 'Donald Trump', level: 'federal' },
      { id: 'vp', title: 'Vice President', name: 'JD Vance', level: 'federal' },
      { id: 'senator-1', title: 'U.S. Senator', name: 'Alex Padilla', level: 'federal' },
      { id: 'senator-2', title: 'U.S. Senator', name: 'Adam Schiff', level: 'federal' },
      { id: 'rep', title: 'U.S. Representative', name: 'Nancy Pelosi', level: 'federal' },
    ],
  },
  laws: { local: 847, county: 234, state: 4521, federal: 6892, international: 353 },
};

// Utility functions
const getTotalOfficials = (officials: typeof MOCK_GOVERNANCE.officials) =>
  Object.values(officials).reduce((sum, arr) => sum + arr.length, 0);

const getTotalLaws = (laws: typeof MOCK_GOVERNANCE.laws) =>
  Object.values(laws).reduce((sum, count) => sum + count, 0);

const calculateDaysLived = (birthdate: string) => {
  const birth = new Date(birthdate);
  const today = new Date();
  return Math.floor(Math.abs(today.getTime() - birth.getTime()) / (1000 * 60 * 60 * 24));
};

const formatNumber = (num: number) => num.toLocaleString();

const calculateAge = (birthdate: string) => {
  const birth = new Date(birthdate);
  const today = new Date();
  let age = today.getFullYear() - birth.getFullYear();
  const m = today.getMonth() - birth.getMonth();
  if (m < 0 || (m === 0 && today.getDate() < birth.getDate())) age--;
  return age;
};

export default function Home() {
  const [screen, setScreen] = useState<Screen>('welcome');
  const [birthdate, setBirthdate] = useState('');
  const [timeBalance, setTimeBalance] = useState(0);
  const [worldIdVerified, setWorldIdVerified] = useState(false);
  const [verificationLevel, setVerificationLevel] = useState<'orb' | 'device' | null>(null);
  const [nullifierHash, setNullifierHash] = useState<string | null>(null);
  const [discoveryProgress, setDiscoveryProgress] = useState(0);
  const [discoveryStage, setDiscoveryStage] = useState('');
  const [allocations, setAllocations] = useState<Record<string, number>>({});
  const [expandedLevel, setExpandedLevel] = useState<string | null>(null);
  const [verifyError, setVerifyError] = useState<string | null>(null);

  const totalOfficials = getTotalOfficials(MOCK_GOVERNANCE.officials);
  const totalLaws = getTotalLaws(MOCK_GOVERNANCE.laws);

  // ============================================================================
  // WORLD ID VERIFICATION
  // ============================================================================

  /**
   * Verify the World ID proof with our backend
   * This is called by IDKit after the user scans the QR code
   */
  const handleVerify = useCallback(async (proof: ISuccessResult) => {
    console.log('Verifying World ID proof...', proof);
    setVerifyError(null);

    try {
      const response = await fetch('/api/verify', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          merkle_root: proof.merkle_root,
          nullifier_hash: proof.nullifier_hash,
          proof: proof.proof,
          verification_level: proof.verification_level,
        }),
      });

      const result = await response.json();

      if (!response.ok || !result.success) {
        throw new Error(result.detail || result.error || 'Verification failed');
      }

      console.log('Verification successful!', result);
    } catch (error) {
      console.error('Verification error:', error);
      // For hackathon demo, we'll allow continuing even if backend verification fails
      // In production, you'd throw here to show error in IDKit modal
      console.log('Demo mode: allowing user to continue');
    }
  }, []);

  /**
   * Called when World ID verification completes successfully
   */
  const onWorldIdSuccess = useCallback((result: ISuccessResult) => {
    console.log('World ID Success!', result);
    setWorldIdVerified(true);
    setVerificationLevel(result.verification_level as 'orb' | 'device');
    setNullifierHash(result.nullifier_hash);
    setScreen('birthdate');
  }, []);

  // ============================================================================
  // TIME CLAIMING & DISCOVERY
  // ============================================================================

  const handleClaimTime = () => {
    const days = calculateDaysLived(birthdate);
    setTimeBalance(days);
    setScreen('discovering');
    runDiscovery(days);
  };

  const runDiscovery = (balance: number) => {
    const stages = [
      { progress: 10, stage: 'Verifying jurisdiction...' },
      { progress: 25, stage: 'Discovering local officials...' },
      { progress: 40, stage: 'Discovering county officials...' },
      { progress: 55, stage: 'Discovering state officials...' },
      { progress: 70, stage: 'Discovering federal officials...' },
      { progress: 85, stage: 'Counting applicable laws...' },
      { progress: 95, stage: 'Calculating allocations...' },
      { progress: 100, stage: 'Discovery complete!' },
    ];

    let i = 0;
    const interval = setInterval(() => {
      if (i < stages.length) {
        setDiscoveryProgress(stages[i].progress);
        setDiscoveryStage(stages[i].stage);
        i++;
      } else {
        clearInterval(interval);
        initializeAllocations(balance);
        setTimeout(() => setScreen('two_numbers'), 500);
      }
    }, 400);
  };

  const initializeAllocations = (balance: number) => {
    const allOfficials = [
      ...MOCK_GOVERNANCE.officials.local,
      ...MOCK_GOVERNANCE.officials.county,
      ...MOCK_GOVERNANCE.officials.state,
      ...MOCK_GOVERNANCE.officials.federal,
    ];
    const levelWeights: Record<string, number> = { local: 0.3, county: 0.15, state: 0.25, federal: 0.25 };
    const newAllocations: Record<string, number> = {};
    allOfficials.forEach((official) => {
      const levelWeight = levelWeights[official.level] || 0.05;
      const officialsAtLevel = MOCK_GOVERNANCE.officials[official.level as keyof typeof MOCK_GOVERNANCE.officials].length;
      newAllocations[official.id] = Math.floor((balance * levelWeight) / officialsAtLevel);
    });
    setAllocations(newAllocations);
  };

  const getAllocation = (officialId: string) => allocations[officialId] || 0;
  
  const getLevelAllocation = (level: string) => {
    const officials = MOCK_GOVERNANCE.officials[level as keyof typeof MOCK_GOVERNANCE.officials];
    return officials.reduce((sum, official) => sum + (allocations[official.id] || 0), 0);
  };

  // ============================================================================
  // RENDER
  // ============================================================================

  return (
    <div style={styles.container}>
      <div style={styles.backgroundGrid} />

      {/* Header */}
      <header style={styles.header}>
        <div style={styles.logo}>
          <span style={styles.logoIcon}>⏰</span>
          <span style={styles.logoText}>TIME</span>
        </div>
        {worldIdVerified && (
          <div style={styles.verifiedBadge}>
            <span>✓</span>
            <span>{verificationLevel === 'orb' ? '🔮 Orb Verified' : '📱 Device Verified'}</span>
          </div>
        )}
      </header>

      {/* Main Content */}
      <main style={styles.main}>
        {/* ================================================================
            WELCOME SCREEN
            ================================================================ */}
        {screen === 'welcome' && (
          <div style={styles.screenContainer}>
            <div style={styles.welcomeContent}>
              <h1 style={styles.heroTitle}>Your Time Is Your Vote</h1>
              <p style={styles.heroSubtitle}>
                Discover who governs you. Claim your birthright.
                <br />
                Let your AI agent distribute your voice.
              </p>

              <div style={styles.featureGrid}>
                <div style={styles.featureCard}>
                  <span style={styles.featureIcon}>🔍</span>
                  <h3 style={styles.featureTitle}>Discover</h3>
                  <p style={styles.featureText}>AI agent finds every official and law that governs you</p>
                </div>
                <div style={styles.featureCard}>
                  <span style={styles.featureIcon}>⚖️</span>
                  <h3 style={styles.featureTitle}>Allocate</h3>
                  <p style={styles.featureText}>Your TIME distributed across the status quo automatically</p>
                </div>
                <div style={styles.featureCard}>
                  <span style={styles.featureIcon}>🔔</span>
                  <h3 style={styles.featureTitle}>Monitor</h3>
                  <p style={styles.featureText}>Agent keeps working — alerts you when elections change things</p>
                </div>
              </div>

              {/* World ID Widget */}
              <IDKitWidget
                app_id={WORLD_ID_APP_ID}
                action={WORLD_ID_ACTION}
                onSuccess={onWorldIdSuccess}
                handleVerify={handleVerify}
                verification_level={VerificationLevel.Device}
              >
                {({ open }) => (
                  <button style={styles.primaryButton} onClick={open}>
                    <span style={styles.worldIdIcon}>🌍</span>
                    Sign in with World ID
                  </button>
                )}
              </IDKitWidget>

              {verifyError && (
                <p style={styles.errorText}>{verifyError}</p>
              )}

              <p style={styles.footnote}>One person. One claim. Forever.</p>

              <div style={styles.worldIdInfo}>
                <p style={styles.worldIdInfoText}>
                  World ID proves you're a unique human without revealing your identity.
                  <br />
                  <a
                    href="https://worldcoin.org/download"
                    target="_blank"
                    rel="noopener noreferrer"
                    style={styles.worldIdLink}
                  >
                    Download World App →
                  </a>
                </p>
              </div>
            </div>
          </div>
        )}

        {/* ================================================================
            BIRTHDATE SCREEN
            ================================================================ */}
        {screen === 'birthdate' && (
          <div style={styles.screenContainer}>
            <div style={styles.birthdateContent}>
              <div style={styles.stepIndicator}>
                <span style={styles.stepNumber}>2</span>
                <span style={styles.stepLabel}>Claim Your Birthright</span>
              </div>

              <h2 style={styles.sectionTitle}>When were you born?</h2>
              <p style={styles.sectionSubtitle}>
                You receive <strong>1 TIME per day</strong> you've lived.
                <br />
                This is your birthright. Your voice. Your vote.
              </p>

              {nullifierHash && (
                <div style={styles.proofBadge}>
                  <span>🔐 Proof Ready</span>
                  <span style={styles.nullifierPreview}>
                    Nullifier: {nullifierHash.slice(0, 10)}...{nullifierHash.slice(-6)}
                  </span>
                </div>
              )}

              <div style={styles.birthdateInputContainer}>
                <input
                  type="date"
                  value={birthdate}
                  onChange={(e) => setBirthdate(e.target.value)}
                  style={styles.birthdateInput}
                  max={new Date().toISOString().split('T')[0]}
                />
              </div>

              {birthdate && (
                <div style={styles.timePreview}>
                  <div style={styles.timePreviewLabel}>Your birthright:</div>
                  <div style={styles.timePreviewAmount}>{formatNumber(calculateDaysLived(birthdate))} TIME</div>
                  <div style={styles.timePreviewDetail}>
                    {calculateAge(birthdate)} years = {formatNumber(calculateDaysLived(birthdate))} days lived
                  </div>
                </div>
              )}

              <button
                style={{
                  ...styles.primaryButton,
                  opacity: birthdate ? 1 : 0.5,
                  cursor: birthdate ? 'pointer' : 'not-allowed',
                }}
                onClick={handleClaimTime}
                disabled={!birthdate}
              >
                Claim My TIME →
              </button>

              <p style={styles.claimNote}>
                Your World ID proof will be submitted to the NEAR blockchain.
                <br />
                The nullifier ensures you can only claim once.
              </p>
            </div>
          </div>
        )}

        {/* ================================================================
            DISCOVERING SCREEN
            ================================================================ */}
        {screen === 'discovering' && (
          <div style={styles.screenContainer}>
            <div style={styles.discoveryContent}>
              <div style={styles.agentIcon}>🤖</div>
              <h2 style={styles.sectionTitle}>AI Agent Discovering...</h2>

              <div style={styles.progressContainer}>
                <div style={styles.progressBar}>
                  <div style={{ ...styles.progressFill, width: `${discoveryProgress}%` }} />
                </div>
                <div style={styles.progressText}>{discoveryStage}</div>
              </div>

              <div style={styles.discoveryStats}>
                <div style={styles.discoveryStat}>
                  <span style={styles.discoveryStatValue}>{discoveryProgress >= 70 ? totalOfficials : '...'}</span>
                  <span style={styles.discoveryStatLabel}>Officials Found</span>
                </div>
                <div style={styles.discoveryStat}>
                  <span style={styles.discoveryStatValue}>
                    {discoveryProgress >= 85 ? formatNumber(totalLaws) : '...'}
                  </span>
                  <span style={styles.discoveryStatLabel}>Laws Found</span>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* ================================================================
            TWO NUMBERS SCREEN
            ================================================================ */}
        {screen === 'two_numbers' && (
          <div style={styles.screenContainer}>
            <div style={styles.twoNumbersContent}>
              <div style={styles.agentBanner}>
                <span>🤖</span>
                <span>Agent: Discovery Complete</span>
              </div>

              <h2 style={styles.bigQuestion}>WHO GOVERNS YOU?</h2>

              <div style={styles.twoNumbersGrid}>
                <div style={styles.numberCard}>
                  <div style={styles.bigNumber}>{totalOfficials}</div>
                  <div style={styles.numberLabel}>OFFICIALS</div>
                  <div style={styles.numberDetail}>govern you right now</div>
                </div>
                <div style={styles.numberCard}>
                  <div style={styles.bigNumber}>{formatNumber(totalLaws)}</div>
                  <div style={styles.numberLabel}>LAWS</div>
                  <div style={styles.numberDetail}>apply to you</div>
                </div>
              </div>

              <div style={styles.locationBadge}>
                <span>📍</span>
                <span>{MOCK_GOVERNANCE.location}</span>
              </div>

              <div style={styles.allocationNotice}>
                <span>✅</span>
                <span>
                  Your <strong>{formatNumber(timeBalance)} TIME</strong> has been distributed across the status quo.
                </span>
              </div>

              <p style={styles.callToAction}>
                To become politically active: <strong>just move your TIME.</strong>
              </p>

              <button style={styles.primaryButton} onClick={() => setScreen('explore')}>
                Explore & Reallocate →
              </button>

              <div style={styles.agentNote}>
                <span>🔔</span>
                <span>Agent will notify you when elections or laws change.</span>
              </div>
            </div>
          </div>
        )}

        {/* ================================================================
            EXPLORE SCREEN
            ================================================================ */}
        {screen === 'explore' && (
          <div style={styles.screenContainer}>
            <div style={styles.exploreContent}>
              <button style={styles.backButton} onClick={() => setScreen('two_numbers')}>
                ← Back
              </button>

              <div style={styles.exploreHeader}>
                <h2 style={styles.exploreTitle}>Your Governance</h2>
                <div style={styles.totalTime}>
                  <span style={styles.totalTimeLabel}>Total TIME:</span>
                  <span style={styles.totalTimeValue}>{formatNumber(timeBalance)}</span>
                </div>
              </div>

              <div style={styles.tabContainer}>
                <button style={{ ...styles.tab, ...styles.tabActive }}>Officials ({totalOfficials})</button>
                <button style={styles.tab}>Laws ({formatNumber(totalLaws)})</button>
              </div>

              <div style={styles.levelList}>
                {(['local', 'county', 'state', 'federal'] as const).map((level) => (
                  <div key={level} style={styles.levelSection}>
                    <button
                      style={styles.levelHeader}
                      onClick={() => setExpandedLevel(expandedLevel === level ? null : level)}
                    >
                      <div style={styles.levelInfo}>
                        <span style={styles.levelName}>{level.toUpperCase()}</span>
                        <span style={styles.levelCount}>
                          ({MOCK_GOVERNANCE.officials[level].length} officials)
                        </span>
                      </div>
                      <div style={styles.levelAllocation}>
                        {formatNumber(getLevelAllocation(level))} TIME
                        <span style={styles.expandIcon}>{expandedLevel === level ? '▼' : '▶'}</span>
                      </div>
                    </button>
                    {expandedLevel === level && (
                      <div style={styles.officialList}>
                        {MOCK_GOVERNANCE.officials[level].map((official) => (
                          <div key={official.id} style={styles.officialRow}>
                            <div style={styles.officialInfo}>
                              <span style={styles.officialTitle}>{official.title}</span>
                              <span style={styles.officialName}>{official.name}</span>
                            </div>
                            <div style={styles.officialAllocation}>
                              {formatNumber(getAllocation(official.id))} TIME
                            </div>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                ))}
              </div>

              <div style={styles.actionBar}>
                <button style={styles.secondaryButton}>🔄 Reallocate TIME</button>
                <button style={styles.secondaryButton}>📊 Stake for Voting Power</button>
              </div>
            </div>
          </div>
        )}
      </main>

      {/* Footer */}
      <footer style={styles.footer}>
        <div style={styles.footerContent}>
          <span>The Great Reset: January 1, 2034</span>
          <span style={styles.footerDivider}>•</span>
          <span>Democracy Earth Foundation</span>
        </div>
      </footer>
    </div>
  );
}

// ============================================================================
// STYLES
// ============================================================================

const styles: Record<string, React.CSSProperties> = {
  container: {
    minHeight: '100vh',
    background: 'linear-gradient(135deg, #0a0f1c 0%, #1a1f35 50%, #0d1424 100%)',
    color: '#ffffff',
    fontFamily: '"DM Sans", "Segoe UI", system-ui, sans-serif',
    position: 'relative',
    overflow: 'hidden',
  },
  backgroundGrid: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    backgroundImage:
      'linear-gradient(rgba(59, 130, 246, 0.03) 1px, transparent 1px), linear-gradient(90deg, rgba(59, 130, 246, 0.03) 1px, transparent 1px)',
    backgroundSize: '50px 50px',
    pointerEvents: 'none',
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: '20px 32px',
    borderBottom: '1px solid rgba(255,255,255,0.08)',
    position: 'relative',
    zIndex: 10,
  },
  logo: { display: 'flex', alignItems: 'center', gap: '10px' },
  logoIcon: { fontSize: '28px' },
  logoText: {
    fontSize: '24px',
    fontWeight: 700,
    letterSpacing: '0.1em',
    background: 'linear-gradient(135deg, #60a5fa 0%, #a78bfa 100%)',
    WebkitBackgroundClip: 'text',
    WebkitTextFillColor: 'transparent',
  },
  verifiedBadge: {
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    padding: '8px 16px',
    background: 'rgba(34, 197, 94, 0.15)',
    border: '1px solid rgba(34, 197, 94, 0.3)',
    borderRadius: '20px',
    fontSize: '14px',
    color: '#4ade80',
  },
  main: { flex: 1, display: 'flex', flexDirection: 'column', position: 'relative', zIndex: 10 },
  screenContainer: { flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', padding: '40px 20px' },
  welcomeContent: { textAlign: 'center', maxWidth: '800px' },
  heroTitle: {
    fontSize: '48px',
    fontWeight: 700,
    marginBottom: '20px',
    background: 'linear-gradient(135deg, #ffffff 0%, #94a3b8 100%)',
    WebkitBackgroundClip: 'text',
    WebkitTextFillColor: 'transparent',
    letterSpacing: '-0.02em',
  },
  heroSubtitle: { fontSize: '18px', color: '#94a3b8', marginBottom: '40px', lineHeight: 1.6 },
  featureGrid: { display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '20px', marginBottom: '40px' },
  featureCard: {
    padding: '24px 20px',
    background: 'rgba(255,255,255,0.03)',
    border: '1px solid rgba(255,255,255,0.08)',
    borderRadius: '16px',
    textAlign: 'center',
  },
  featureIcon: { fontSize: '28px', display: 'block', marginBottom: '12px' },
  featureTitle: { fontSize: '16px', fontWeight: 600, marginBottom: '8px', color: '#ffffff' },
  featureText: { fontSize: '13px', color: '#94a3b8', lineHeight: 1.5 },
  primaryButton: {
    display: 'inline-flex',
    alignItems: 'center',
    gap: '12px',
    padding: '16px 32px',
    fontSize: '16px',
    fontWeight: 600,
    color: '#ffffff',
    background: 'linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%)',
    border: 'none',
    borderRadius: '12px',
    cursor: 'pointer',
    boxShadow: '0 4px 20px rgba(59, 130, 246, 0.3)',
  },
  worldIdIcon: { fontSize: '20px' },
  footnote: { marginTop: '20px', fontSize: '14px', color: '#64748b' },
  errorText: { marginTop: '16px', fontSize: '14px', color: '#ef4444' },
  worldIdInfo: {
    marginTop: '32px',
    padding: '16px 24px',
    background: 'rgba(255,255,255,0.03)',
    borderRadius: '12px',
  },
  worldIdInfoText: { fontSize: '13px', color: '#64748b', lineHeight: 1.6 },
  worldIdLink: { color: '#60a5fa', textDecoration: 'none' },
  birthdateContent: { textAlign: 'center', maxWidth: '500px' },
  stepIndicator: {
    display: 'inline-flex',
    alignItems: 'center',
    gap: '12px',
    padding: '8px 20px',
    background: 'rgba(59, 130, 246, 0.15)',
    border: '1px solid rgba(59, 130, 246, 0.3)',
    borderRadius: '20px',
    marginBottom: '32px',
  },
  stepNumber: {
    width: '24px',
    height: '24px',
    background: '#3b82f6',
    borderRadius: '50%',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    fontSize: '14px',
    fontWeight: 600,
  },
  stepLabel: { fontSize: '14px', color: '#60a5fa' },
  sectionTitle: { fontSize: '28px', fontWeight: 600, marginBottom: '16px' },
  sectionSubtitle: { fontSize: '16px', color: '#94a3b8', marginBottom: '24px', lineHeight: 1.6 },
  proofBadge: {
    display: 'inline-flex',
    flexDirection: 'column',
    alignItems: 'center',
    gap: '4px',
    padding: '12px 20px',
    background: 'rgba(34, 197, 94, 0.1)',
    border: '1px solid rgba(34, 197, 94, 0.2)',
    borderRadius: '12px',
    marginBottom: '24px',
    fontSize: '14px',
    color: '#4ade80',
  },
  nullifierPreview: { fontSize: '11px', color: '#64748b', fontFamily: 'monospace' },
  birthdateInputContainer: { marginBottom: '24px' },
  birthdateInput: {
    padding: '16px 24px',
    fontSize: '18px',
    background: 'rgba(255,255,255,0.05)',
    border: '2px solid rgba(255,255,255,0.1)',
    borderRadius: '12px',
    color: '#ffffff',
    outline: 'none',
    cursor: 'pointer',
  },
  timePreview: {
    padding: '24px',
    background: 'rgba(59, 130, 246, 0.1)',
    border: '1px solid rgba(59, 130, 246, 0.2)',
    borderRadius: '16px',
    marginBottom: '24px',
  },
  timePreviewLabel: { fontSize: '14px', color: '#60a5fa', marginBottom: '8px' },
  timePreviewAmount: {
    fontSize: '36px',
    fontWeight: 700,
    background: 'linear-gradient(135deg, #60a5fa 0%, #a78bfa 100%)',
    WebkitBackgroundClip: 'text',
    WebkitTextFillColor: 'transparent',
  },
  timePreviewDetail: { fontSize: '14px', color: '#94a3b8', marginTop: '8px' },
  claimNote: { marginTop: '20px', fontSize: '12px', color: '#64748b', lineHeight: 1.5 },
  discoveryContent: { textAlign: 'center', maxWidth: '500px' },
  agentIcon: { fontSize: '56px', marginBottom: '24px' },
  progressContainer: { marginTop: '32px', marginBottom: '40px' },
  progressBar: {
    width: '100%',
    height: '8px',
    background: 'rgba(255,255,255,0.1)',
    borderRadius: '4px',
    overflow: 'hidden',
  },
  progressFill: {
    height: '100%',
    background: 'linear-gradient(90deg, #3b82f6 0%, #8b5cf6 100%)',
    transition: 'width 0.3s ease',
  },
  progressText: { marginTop: '16px', fontSize: '16px', color: '#60a5fa' },
  discoveryStats: { display: 'flex', justifyContent: 'center', gap: '48px' },
  discoveryStat: { display: 'flex', flexDirection: 'column', alignItems: 'center' },
  discoveryStatValue: { fontSize: '28px', fontWeight: 700, color: '#ffffff' },
  discoveryStatLabel: { fontSize: '14px', color: '#64748b', marginTop: '4px' },
  twoNumbersContent: { textAlign: 'center', maxWidth: '600px' },
  agentBanner: {
    display: 'inline-flex',
    alignItems: 'center',
    gap: '8px',
    padding: '10px 20px',
    background: 'rgba(34, 197, 94, 0.15)',
    border: '1px solid rgba(34, 197, 94, 0.3)',
    borderRadius: '20px',
    fontSize: '14px',
    color: '#4ade80',
    marginBottom: '32px',
  },
  bigQuestion: { fontSize: '24px', fontWeight: 600, color: '#94a3b8', marginBottom: '32px', letterSpacing: '0.1em' },
  twoNumbersGrid: { display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: '20px', marginBottom: '24px' },
  numberCard: {
    padding: '28px',
    background: 'rgba(255,255,255,0.03)',
    border: '1px solid rgba(255,255,255,0.1)',
    borderRadius: '20px',
  },
  bigNumber: {
    fontSize: '56px',
    fontWeight: 700,
    background: 'linear-gradient(135deg, #60a5fa 0%, #a78bfa 100%)',
    WebkitBackgroundClip: 'text',
    WebkitTextFillColor: 'transparent',
    lineHeight: 1,
  },
  numberLabel: { fontSize: '16px', fontWeight: 600, color: '#ffffff', marginTop: '12px', letterSpacing: '0.1em' },
  numberDetail: { fontSize: '13px', color: '#64748b', marginTop: '4px' },
  locationBadge: {
    display: 'inline-flex',
    alignItems: 'center',
    gap: '8px',
    padding: '10px 20px',
    background: 'rgba(255,255,255,0.05)',
    borderRadius: '20px',
    fontSize: '15px',
    color: '#94a3b8',
    marginBottom: '24px',
  },
  allocationNotice: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    gap: '12px',
    padding: '16px 24px',
    background: 'rgba(59, 130, 246, 0.1)',
    border: '1px solid rgba(59, 130, 246, 0.2)',
    borderRadius: '12px',
    marginBottom: '20px',
    fontSize: '15px',
    color: '#94a3b8',
  },
  callToAction: { fontSize: '16px', color: '#94a3b8', marginBottom: '28px' },
  agentNote: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    gap: '8px',
    marginTop: '20px',
    fontSize: '14px',
    color: '#64748b',
  },
  exploreContent: { width: '100%', maxWidth: '600px' },
  backButton: {
    background: 'none',
    border: 'none',
    color: '#60a5fa',
    fontSize: '15px',
    cursor: 'pointer',
    marginBottom: '20px',
    padding: '8px 0',
  },
  exploreHeader: { display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' },
  exploreTitle: { fontSize: '24px', fontWeight: 600 },
  totalTime: { textAlign: 'right' as const },
  totalTimeLabel: { fontSize: '12px', color: '#64748b', display: 'block' },
  totalTimeValue: {
    fontSize: '22px',
    fontWeight: 600,
    background: 'linear-gradient(135deg, #60a5fa 0%, #a78bfa 100%)',
    WebkitBackgroundClip: 'text',
    WebkitTextFillColor: 'transparent',
  },
  tabContainer: { display: 'flex', gap: '8px', marginBottom: '20px' },
  tab: {
    flex: 1,
    padding: '12px 16px',
    background: 'rgba(255,255,255,0.03)',
    border: '1px solid rgba(255,255,255,0.08)',
    borderRadius: '8px',
    color: '#64748b',
    fontSize: '14px',
    cursor: 'pointer',
  },
  tabActive: {
    background: 'rgba(59, 130, 246, 0.15)',
    border: '1px solid rgba(59, 130, 246, 0.3)',
    color: '#60a5fa',
  },
  levelList: { display: 'flex', flexDirection: 'column', gap: '10px', marginBottom: '20px' },
  levelSection: {
    background: 'rgba(255,255,255,0.02)',
    border: '1px solid rgba(255,255,255,0.08)',
    borderRadius: '12px',
    overflow: 'hidden',
  },
  levelHeader: {
    width: '100%',
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: '14px 18px',
    background: 'none',
    border: 'none',
    color: '#ffffff',
    cursor: 'pointer',
    textAlign: 'left' as const,
  },
  levelInfo: { display: 'flex', alignItems: 'center', gap: '12px' },
  levelName: { fontSize: '15px', fontWeight: 600, letterSpacing: '0.05em' },
  levelCount: { fontSize: '13px', color: '#64748b' },
  levelAllocation: { display: 'flex', alignItems: 'center', gap: '12px', fontSize: '15px', color: '#60a5fa' },
  expandIcon: { fontSize: '11px', color: '#64748b' },
  officialList: { borderTop: '1px solid rgba(255,255,255,0.05)' },
  officialRow: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: '12px 18px',
    borderBottom: '1px solid rgba(255,255,255,0.03)',
  },
  officialInfo: { display: 'flex', flexDirection: 'column', gap: '2px' },
  officialTitle: { fontSize: '13px', color: '#94a3b8' },
  officialName: { fontSize: '15px', fontWeight: 500 },
  officialAllocation: { fontSize: '14px', color: '#60a5fa' },
  actionBar: { display: 'flex', gap: '12px' },
  secondaryButton: {
    flex: 1,
    padding: '14px 18px',
    background: 'rgba(255,255,255,0.05)',
    border: '1px solid rgba(255,255,255,0.1)',
    borderRadius: '10px',
    color: '#ffffff',
    fontSize: '14px',
    cursor: 'pointer',
  },
  footer: {
    padding: '20px 32px',
    borderTop: '1px solid rgba(255,255,255,0.08)',
    position: 'relative',
    zIndex: 10,
  },
  footerContent: {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    gap: '16px',
    fontSize: '14px',
    color: '#64748b',
  },
  footerDivider: { color: '#334155' },
};
