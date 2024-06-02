import { ethers } from 'ethers'
import BigNumber from 'bignumber.js'

export function isAddress(value: string) {
  return ethers.isAddress(value)
}

export function isBytesLike(value: string) {
  return ethers.isBytesLike(value)
}

export function isBytes(value: string, bytes: bigint) {
  return ethers.isBytesLike(value) && ethers.toBigInt(value.replace('0x', '').length) === bytes * 2n
}

export function formatPercent(value: string | bigint, precision = 2) {
  return isNaN(Number(value.toString())) ? '0 %' : `${formatNumber(value, precision)} %`
}

export function formatNumber(value: string | bigint, precision = 4) {
  return new BigNumber(value.toString()).decimalPlaces(precision, BigNumber.ROUND_DOWN).toFormat()
}

export function singlePrecision(amount?: string | bigint) {
  if (!amount) {
    return '0'
  }

  const value = new BigNumber(String(amount)).div(String(10n ** 25n))

  return value.mod(1).gte(0.5)
    ? value.integerValue(BigNumber.ROUND_UP).toString()
    : value.integerValue(BigNumber.ROUND_DOWN).toString()
}
