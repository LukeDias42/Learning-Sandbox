import Data.Bits (xor, shiftR)
import Data.Word (Word64)

firstShift :: Word64 -> Word64
firstShift z = (xor z (shiftR z 30)) * 0xbf58476d1ce4e5b9

secondShift :: Word64 -> Word64
secondShift z = (xor z (shiftR z 27)) * 0x94d049bb133111eb

updateState :: Word64 -> Word64 -> Word64
updateState state n = state + 0x9e3779b97f4a7c15 * n

next :: Word64 -> Word64 -> Word64
next state iterations =
  let z = secondShift (firstShift (updateState state iterations))
  in xor z (shiftR z 31)
