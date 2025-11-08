export class User {
  login: string
  img_url: string
  grade: UserGrade

  constructor(login: string, img_url: string, grade: string) {
    this.login = login
    this.img_url = img_url
    this.grade = gradeFromString(grade)
  }
}

export enum UserGrade {
  Interested,
  Novice,
  Member,
  Bartender,
  Coordinator,
  HonoraryMember,
  President,
  Unknown,
}

function gradeFromString(str: string): UserGrade {
  switch (str) {
    case 'Interested':
      return UserGrade.Interested
    case 'Novice':
      return UserGrade.Novice
    case 'Member':
      return UserGrade.Member
    case 'Bartender':
      return UserGrade.Bartender
    case 'Coordinator':
      return UserGrade.Coordinator
    case 'HonoraryMember':
      return UserGrade.HonoraryMember
    case 'President':
      return UserGrade.President
    default:
      return UserGrade.Unknown
  }
}
