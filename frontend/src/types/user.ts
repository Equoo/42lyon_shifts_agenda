export class User {
  login: String
  img_url: String
  grade: UserGrade

  constructor(login: String, img_url: String, grade: String) {
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

function gradeFromString(str: String): UserGrade {
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
