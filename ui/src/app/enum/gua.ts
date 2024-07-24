export enum GuaName {
  坎 = '坎',
  艮 = '艮',
  震 = '震',
  巽 = '巽',
  离 = '离',
  坤 = '坤',
  兑 = '兑',
  乾 = '乾',
}

export function GuaNum(name: GuaName) {
  switch (name) {
    case GuaName.坎:
      return '一';
    case GuaName.艮:
      return '八';
    case GuaName.震:
      return '三';
    case GuaName.巽:
      return '四';
    case GuaName.离:
      return '九';
    case GuaName.坤:
      return '二';
    case GuaName.兑:
      return '七';
    case GuaName.乾:
      return '六';
  }
}
