package repository

import scalikejdbc._

import scala.util.{Failure, Success, Try}

class {{ RepositoryName }} extends Repository[{{ EntityName }}] {
  import {{ EntityName }}._

  val {{ TableAlias }} = {{ ModelName }}.column

  override def find(id: Int): Option[{{ EntityName }}] = {
    {{ ModelName }}.find(id) match {
      case Some(model) => Some(model)
      case None => None
    }
  }

  {% set i = 0 -%}
  {% set suffix = "," -%}
  override def create(entity: {{ EntityName }}): Either[Exception, {{ EntityName }}] = {
    Try {
      DB localTx {implicit session =>
        withSQL {
          insert.into({{ ModelName }}).namedValues(
          {%for column in column_list -%}
             {% set i = i + 1 -%}
             {%if i >= column_list_length -%}
             {% set suffix = "" -%}
             {%endif -%}
             {%if column.column_name_camel != "id" -%}
             {{ TableAlias }}.{{ column.column_name_camel }} -> entity.{{ column.column_name_camel }}{{ suffix }}
             {%endif -%}
          {% endfor -%}
          )
        }.update().apply()
      }
    } match {
      case Success(_) => Right(entity)
      case Failure(e) => Left(new Exception(e))
    }
  }

  {% set i = 0 -%}
  {% set suffix = "," -%}
  override def update(entity: {{ EntityName }}): Either[Exception, {{ EntityName }}] = {
    {{ ModelName }}.find(entity.id) match {
      case Some(model) =>
        model.copy(
          {%for column in column_list -%}
            {% set i = i + 1 -%}
            {%if i >= column_list_length -%}
              {% set suffix = "" -%}
            {%endif -%}
            {%if column.column_name_camel != "id" -%}
          {{ column.column_name_camel }} = entity.{{ column.column_name_camel }}{{ suffix }}
            {%endif -%}
          {% endfor -%}
        ).save()
        Right(model)

      case None => Left(new Exception(""))
    }
  }

  override def destroy(id: Int): Option[{{ EntityName }}] = {
    {{ ModelName }}.find(id) match {
      case Some(model) =>
        model.destroy()
        Some(model)

      case None => None
    }
  }

}
