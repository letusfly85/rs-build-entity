package entity

import play.api.libs.functional.syntax._
import play.api.libs.json.Reads._
import play.api.libs.json._

final case class {{ entityName }}Entity (
{%for column in column_list %}  {{column.columnNameCamel}}: {{column.dataType}},
  {% endfor %}
) extends Entity

object {{ entityName }} {
  implicit def {{ table_name }}Reads: Reads[{{ entityName }}Entity] = (
  {%for column in column_list %}  (JsPath \ "{{column.columnName}}").read[{{column.dataType}}] and
  {% endfor %} )({{ entityName }}Entity.apply _)

  implicit def {{ table_name }}Writes: Writes[{{ entityName }}Entity] = (
  {%for column in column_list %}  (JsPath \ "{{column.columnName}}").write[{{column.dataType}}] and
  {% endfor %} )(unlift({{ entityName }}Entity.unapply))
}

