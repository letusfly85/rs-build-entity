package entity

import play.api.libs.functional.syntax._
import play.api.libs.json.Reads._
import play.api.libs.json._

final case class {{ EntityName }} (
{%for column in column_list -%}
  {%if column.is_nullable == "YES" -%}
    {{ column.columnNameCamel }}: Option[{{ column.dataType }}],
  {%else -%}
    {{ column.columnNameCamel }}: {{ column.dataType }},
  {%endif -%}
{% endfor -%}) extends Entity

object {{ EntityName }} {
  implicit def {{ camelCaseName }}Reads: Reads[{{ EntityName }}] = (
  {%for column in column_list %}  (JsPath \ "{{column.columnName}}").read[{{column.dataType}}] and
  {% endfor %} )({{ EntityName }}Entity.apply _)

  implicit def {{ camelCaseName }}Writes: Writes[{{ EntityName }}] = (
  {%for column in column_list %}  (JsPath \ "{{column.columnName}}").write[{{column.dataType}}] and
  {% endfor %} )(unlift({{ EntityName }}.unapply))
}

