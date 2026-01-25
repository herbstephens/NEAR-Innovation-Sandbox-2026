// =============================================================================
// World ID Verification API Route
// TIME Protocol - NEARCON Innovation Sandbox
// =============================================================================

import { NextRequest, NextResponse } from 'next/server';

// World ID Cloud verification endpoint
const WORLD_ID_VERIFY_URL = 'https://developer.worldcoin.org/api/v1/verify';

interface WorldIDProof {
  merkle_root: string;
  nullifier_hash: string;
  proof: string;
  verification_level: 'orb' | 'device';
}

interface VerifyRequest extends WorldIDProof {
  signal?: string;
}

export async function POST(request: NextRequest) {
  try {
    const body: VerifyRequest = await request.json();
    
    const { merkle_root, nullifier_hash, proof, verification_level, signal } = body;

    // Validate required fields
    if (!merkle_root || !nullifier_hash || !proof) {
      return NextResponse.json(
        { success: false, error: 'Missing required proof fields' },
        { status: 400 }
      );
    }

    // Get app configuration from environment
    const app_id = process.env.WORLD_ID_APP_ID || process.env.NEXT_PUBLIC_WORLD_ID_APP_ID;
    const action = process.env.WORLD_ID_ACTION || process.env.NEXT_PUBLIC_WORLD_ID_ACTION;

    if (!app_id || !action) {
      console.error('World ID configuration missing');
      return NextResponse.json(
        { success: false, error: 'Server configuration error' },
        { status: 500 }
      );
    }

    // Call World ID verification API
    const verifyResponse = await fetch(WORLD_ID_VERIFY_URL + '/' + app_id, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        merkle_root,
        nullifier_hash,
        proof,
        action,
        signal: signal || '',
      }),
    });

    const verifyResult = await verifyResponse.json();

    if (verifyResponse.ok && verifyResult.success) {
      // Verification successful!
      // The nullifier_hash is unique per user+action combo
      // Store this to prevent duplicate claims
      
      console.log('World ID verification successful:', {
        nullifier_hash,
        verification_level,
        action,
      });

      return NextResponse.json({
        success: true,
        nullifier_hash,
        verification_level,
        message: 'Human verified successfully',
      });
    } else {
      // Verification failed
      console.error('World ID verification failed:', verifyResult);
      
      return NextResponse.json(
        { 
          success: false, 
          error: verifyResult.code || 'verification_failed',
          detail: verifyResult.detail || 'Verification failed',
        },
        { status: 400 }
      );
    }
  } catch (error) {
    console.error('Verification error:', error);
    return NextResponse.json(
      { success: false, error: 'Internal server error' },
      { status: 500 }
    );
  }
}
