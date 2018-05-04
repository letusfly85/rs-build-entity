package repository

import scalikejdbc._

class {{ RepositoryName }} extends Repository[{{ EntityName }}] {

  override def find(id: Int): Option[{{ EntityName }}] = ???

  override def create(entity: {{ EntityName }}): Either[Exception, {{ EntityName }}] = ???

  override def update(entity: {{ EntityName }}): Either[Exception, {{ EntityName }}] = ???

  override def destroy(id: Int): Option[{{ EntityName }}] = ???

}
