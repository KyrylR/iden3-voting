import { ErrorHandler } from '@/helpers'

import { ProofRequestBody, ProofResponse } from '@/types/secrets'
import { config } from '@config'

export async function fetchProof(
  commitment: string,
): Promise<ProofResponse | null> {
  const requestBody: ProofRequestBody = {
    commitment: commitment,
  }

  try {
    const response = await fetch(`${config.APP_API_URL}/api/proof`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(requestBody),
    })

    if (!response.ok) {
      ErrorHandler.process(`HTTP error! status: ${response.status}`)
    }

    return await response.json()
  } catch (error) {
    ErrorHandler.process(error)

    return null
  }
}
