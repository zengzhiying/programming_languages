
#ifndef HALF_HALF_HPP_
#define HALF_HALF_HPP_

// source from: 
// https://github.com/mitsuba-renderer/openexr/blob/master/IlmBase/Half/half.cpp
// https://github.com/mitsuba-renderer/openexr/blob/master/IlmBase/Half/toFloat.cpp

//---------------------------------------------------
// Interpret an unsigned short bit pattern as a half,
// and convert that half to the corresponding float's
// bit pattern.
//---------------------------------------------------

unsigned int halfToFloat (unsigned short y)
{
    int s = (y >> 15) & 0x00000001;
    int e = (y >> 10) & 0x0000001f;
    int m =  y        & 0x000003ff;

    if (e == 0)
    {
	if (m == 0)
	{
	    //
	    // Plus or minus zero
	    //

	    return s << 31;
	}
	else
	{
	    //
	    // Denormalized number -- renormalize it
	    //

	    while (!(m & 0x00000400))
	    {
		m <<= 1;
		e -=  1;
	    }

	    e += 1;
	    m &= ~0x00000400;
	}
    }
    else if (e == 31)
    {
	if (m == 0)
	{
	    //
	    // Positive or negative infinity
	    //

	    return (s << 31) | 0x7f800000;
	}
	else
	{
	    //
	    // Nan -- preserve sign and significand bits
	    //

	    return (s << 31) | 0x7f800000 | (m << 13);
	}
    }

    //
    // Normalized number
    //

    e = e + (127 - 15);
    m = m << 13;

    //
    // Assemble s, e and m.
    //

    return (s << 31) | (e << 23) | m;
}

float overflow ()
{
    volatile float f = 1e10;

    for (int i = 0; i < 10; i++)	
        f *= f;				// this will overflow before
					// the for­loop terminates
    return f;
}

//-----------------------------------------------------
// Float-to-half conversion -- general case, including
// zeroes, denormalized numbers and exponent overflows.
//-----------------------------------------------------

unsigned short floatToHalf (int i)
{
    //
    // Our floating point number, f, is represented by the bit
    // pattern in integer i.  Disassemble that bit pattern into
    // the sign, s, the exponent, e, and the significand, m.
    // Shift s into the position where it will go in in the
    // resulting half number.
    // Adjust e, accounting for the different exponent bias
    // of float and half (127 versus 15).
    //

    int s =  (i >> 16) & 0x00008000;
    int e = ((i >> 23) & 0x000000ff) - (127 - 15);
    int m =   i        & 0x007fffff;

    //
    // Now reassemble s, e and m into a half:
    //

    if (e <= 0)
    {
        if (e < -10)
        {
            //
            // E is less than -10.  The absolute value of f is
            // less than HALF_MIN (f may be a small normalized
            // float, a denormalized float or a zero).
            //
            // We convert f to a half zero with the same sign as f.
            //

            return s;
        }

        //
        // E is between -10 and 0.  F is a normalized float
        // whose magnitude is less than HALF_NRM_MIN.
        //
        // We convert f to a denormalized half.
        //

        //
        // Add an explicit leading 1 to the significand.
        // 

        m = m | 0x00800000;

        //
        // Round to m to the nearest (10+e)-bit value (with e between
        // -10 and 0); in case of a tie, round to the nearest even value.
        //
        // Rounding may cause the significand to overflow and make
        // our number normalized.  Because of the way a half's bits
        // are laid out, we don't have to treat this case separately;
        // the code below will handle it correctly.
        // 

        int t = 14 - e;
        int a = (1 << (t - 1)) - 1;
        int b = (m >> t) & 1;

        m = (m + a + b) >> t;

        //
        // Assemble the half from s, e (zero) and m.
        //

        return s | m;
    }
    else if (e == 0xff - (127 - 15))
    {
        if (m == 0)
        {
            //
            // F is an infinity; convert f to a half
            // infinity with the same sign as f.
            //

            return s | 0x7c00;
        }
        else
        {
            //
            // F is a NAN; we produce a half NAN that preserves
            // the sign bit and the 10 leftmost bits of the
            // significand of f, with one exception: If the 10
            // leftmost bits are all zero, the NAN would turn 
            // into an infinity, so we have to set at least one
            // bit in the significand.
            //

            m >>= 13;
            return s | 0x7c00 | m | (m == 0);
        }
    }
    else
    {
        //
        // E is greater than zero.  F is a normalized float.
        // We try to convert f to a normalized half.
        //

        //
        // Round to m to the nearest 10-bit value.  In case of
        // a tie, round to the nearest even value.
        //

        m = m + 0x00000fff + ((m >> 13) & 1);

        if (m & 0x00800000)
        {
            m =  0;		// overflow in significand,
            e += 1;		// adjust exponent
        }

        //
        // Handle exponent overflow
        //

        if (e > 30)
        {
            overflow ();	// Cause a hardware floating point overflow;
            return s | 0x7c00;	// if this returns, the half becomes an
        }   			// infinity with the same sign as f.

        //
        // Assemble the half from s, e and m.
        //

        return s | (e << 10) | (m >> 13);
    }
}

#endif
