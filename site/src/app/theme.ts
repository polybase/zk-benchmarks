import { extendTheme } from '@chakra-ui/react'

const mode = (light: any, _dark: any) => ({ default: light, _dark })

export const theme = extendTheme({
    semanticTokens: {
        colors: {
            error: 'red.500',
            warning: mode('#ca4b03c7', '#cc630887'),
            'bws': mode('rgba(255, 255, 255)', 'rgba(26, 32, 44)'),
            'bws.100': mode('rgba(240, 240, 240)', 'rgba(29, 31, 36)'),
            'bw.10': mode('rgba(0, 0, 0, 0.01)', 'rgba(255, 255, 255, 0.01)'),
            'bw.50': mode('rgba(0, 0, 0, 0.04)', 'rgba(255, 255, 255, 0.04)'),
            'bw.100': mode('rgba(0, 0, 0, 0.06)', 'rgba(255, 255, 255, 0.06)'),
            'bw.200': mode('rgba(0, 0, 0, 0.08)', 'rgba(255, 255, 255, 0.08)'),
            'bw.300': mode('rgba(0, 0, 0, 0.16)', 'rgba(255, 255, 255, 0.16)'),
            'bw.400': mode('rgba(0, 0, 0, 0.24)', 'rgba(255, 255, 255, 0.24)'),
            'bw.500': mode('rgba(0, 0, 0, 0.36)', 'rgba(255, 255, 255, 0.36)'),
            'bw.600': mode('rgba(0, 0, 0, 0.48)', 'rgba(255, 255, 255, 0.48)'),
            'bw.700': mode('rgba(0, 0, 0, 0.64)', 'rgba(255, 255, 255, 0.64)'),
            'bw.800': mode('rgba(0, 0, 0, 0.80)', 'rgba(255, 255, 255, 0.80)'),
            'bw.900': mode('rgba(0, 0, 0, 0.92)', 'rgba(255, 255, 255, 0.92)'),
        },
    },
})