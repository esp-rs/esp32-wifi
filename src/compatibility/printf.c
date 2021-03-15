#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <limits.h>

extern int _print(const char *s);
extern void abort(void);

typedef enum {
    ITOA_BASE10 = (1 << 0),
    ITOA_SIGNED = (1 << 1),
    ITOA_ZEROPAD = (1 << 2),
    ITOA_UPPERCASE = (1 << 3),
} ItoaFlags;

static char *esp_itoa(char *dst, size_t n, int num, unsigned int width, ItoaFlags flags) {
    unsigned int base = (flags & ITOA_BASE10) ? 10 : 16;
    char fillchar = (flags & ITOA_ZEROPAD) ? '0' : ' ';
    unsigned int absnum;
    char *out = dst;
    char *endout = dst + n;
    char tmp[64] = { 0, };
    char *p = tmp;
    int width_left;

    if (flags & ITOA_SIGNED) {
        absnum = num < 0 ? -num : num;
    } else {
        absnum = (unsigned int)num;
    }

    do {
        int rem = absnum % base;
        if (rem < 10) {
            *p = rem + '0';
        } else {
            *p = rem + ((flags & ITOA_UPPERCASE) ? 'A' : 'a') - 0xa;
        }
        p++;
        absnum = absnum / base;
    } while (absnum != 0 && p <= tmp + sizeof(tmp));

    if ((flags & ITOA_SIGNED) && num < 0) {
        *out = '-';
        out++;
    }

    width_left = width - (p - tmp);
    while (width_left > 0 && out < endout - 1) {
        *out = fillchar;
        out++;
        width_left--;
    }

    do {
        p--;
        *out = *p;
        out++;
    } while (tmp < p);

    *out = '\0';

    return out;
}

int esp_vsnprintf(char *dst, size_t n, const char *fmt, va_list ap) {
    const char *in = fmt;
    char *out = dst;
    const char *endout = dst + n;

    while (*in != '\0' && out < endout - 1) {
        ItoaFlags flags = 0;
        unsigned int width = 0;

        switch (*in) {
            case '%':
start_fmtchar:
                in++;
                switch (*in) {
                    case '%':
                        *out = '%';
                        out++;
                        break;
                    case 'c':
                        *out = va_arg(ap, int);
                        out++;
                        break;
                    case 'd':
                        out = esp_itoa(out, endout - out, va_arg(ap, int), width, flags | ITOA_BASE10 | ITOA_SIGNED);
                        break;
                    case 'u':
                        out = esp_itoa(out, endout - out, va_arg(ap, unsigned int), width, flags | ITOA_BASE10);
                        break;
                    case 'X':
                        flags |= ITOA_UPPERCASE;
                        /* FALLTHROUGH */
                    case 'x':
                        out = esp_itoa(out, endout - out, va_arg(ap, unsigned int), width, flags);
                        break;
                    case 's': {
                        char *tmp = va_arg(ap, char *);
                        while (*tmp != '\0' && out < endout - 1) {
                            *out = *tmp;
                            out++;
                            tmp++;
                        }
                        break;
                    }
                    case '0':
                        if (width == 0) {
                            flags |= ITOA_ZEROPAD;
                        } else {
                            width = width * 10;
                        }
                        goto start_fmtchar;
                    case '*':
                        width = va_arg(ap, unsigned int);
                        goto start_fmtchar;
                    case '\0':
                        goto done;
                    default:
                        if (*in >= '1' && *in <= '9') {
                            width = width * 10 + *in - '0';
                            goto start_fmtchar;
                        } else {
                            abort();
                            return 0;  /* NOTREACHED */
                        }
                }
                break;

            default:
                *out = *in;
                out++;
                break;
        }

        in++;
    }

done:
    *out = '\0';

    return (size_t)(out - dst);
}

int esp_snprintf(char *dst, size_t n, const char *fmt, ...) {
    int ret;
    va_list ap;
    va_start(ap, fmt);
    ret = esp_vsnprintf(dst, n, fmt, ap);
    va_end(ap);
    return ret;
}

int esp_vprintf(const char *fmt, va_list ap) {
    char buf[512];
    esp_vsnprintf(buf, sizeof(buf), fmt, ap);
    return _print(buf);
}

int esp_printf(const char *fmt, ...) {
    int ret;
    va_list ap;
    va_start(ap, fmt);
    ret = esp_vprintf(fmt, ap);
    va_end(ap);
    return ret;
}

/* implicits */

int sprintf(char *s, const char *fmt, ...) {
    int ret;
    va_list ap;
    va_start(ap, fmt);
    ret = esp_vsnprintf(s, INT_MAX, fmt, ap);
    va_end(ap);
    return ret;
}

int phy_printf(const char *fmt, ...) {
    int ret;
    va_list ap;
    va_start(ap, fmt);
    ret = esp_vprintf(fmt, ap);
    va_end(ap);
    return ret;
}

int net80211_printf(const char *fmt, ...) {
    int ret;
    va_list ap;
    va_start(ap, fmt);
    ret = esp_vprintf(fmt, ap);
    va_end(ap);
    return ret;
}

/* OSI */

void _log_writev(unsigned int level, const char *tag, const char *fmt, va_list ap) {
    esp_vprintf(fmt, ap);
}

void _log_write(unsigned int level, const char *tag, const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    _log_writev(level, tag, fmt, ap);
    va_end(ap);
}
